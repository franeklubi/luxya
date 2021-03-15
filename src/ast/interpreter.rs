use crate::ast::{env::*, expr::*, stmt::*};
use crate::token::*;

use std::{fmt, rc::Rc};


pub struct RuntimeError {
	pub message: String,
	pub token: Token,
}

#[derive(Clone)]
pub struct DeclaredValue {
	pub mutable: bool,
	pub value: InterpreterValue,
}

// IDEAS TODO:
// - InterpreterValue should know if it's a child of some identifier or smth
// - If it is /\, then we can print the identifier, rather than it's value
// - Should mimic LiteralValue's fields
#[derive(Clone, PartialEq)]
pub enum InterpreterValue {
	Function {
		fun: Rc<InterpreterFunction>,
		enclosing_env: WrappedInterpreterEnvironment,
	},
	String(Rc<str>),
	Number(f64),
	True,
	False,
	Nil,
}

impl InterpreterValue {
	pub fn to_human_readable(&self) -> &str {
		match self {
			InterpreterValue::Function { .. } => "function",
			InterpreterValue::String(_) => "string",
			InterpreterValue::Number(_) => "number",
			InterpreterValue::True => "boolean",
			InterpreterValue::False => "boolean",
			InterpreterValue::Nil => "nil value",
		}
	}
}

pub enum InterpreterStmtValue {
	Return {
		keyword: Token,
		value: InterpreterValue,
	},
	Break(Token),
	Continue(Token),
	Noop,
}

pub enum InterpreterFunction {
	Native {
		arity: usize,
		fun: fn(
			&WrappedInterpreterEnvironment,
			&[InterpreterValue],
		) -> Result<InterpreterValue, RuntimeError>,
	},
	LoxDefined(FunctionValue),
}

impl PartialEq for InterpreterFunction {
	fn eq(&self, other: &Self) -> bool {
		match (&self, &other) {
			(
				InterpreterFunction::LoxDefined(FunctionValue {
					body: Some(body1),
					..
				}),
				InterpreterFunction::LoxDefined(FunctionValue {
					body: Some(body2),
					..
				}),
			) => Rc::ptr_eq(body1, body2),
			_ => false,
		}
	}
}


impl From<bool> for InterpreterValue {
	fn from(v: bool) -> Self {
		if v {
			InterpreterValue::True
		} else {
			InterpreterValue::False
		}
	}
}

impl From<LiteralValue> for InterpreterValue {
	fn from(v: LiteralValue) -> Self {
		match v {
			LiteralValue::String(s) => InterpreterValue::String(Rc::clone(&s)),
			LiteralValue::Number(n) => InterpreterValue::Number(n),
			LiteralValue::True => InterpreterValue::True,
			LiteralValue::False => InterpreterValue::False,
			LiteralValue::Nil => InterpreterValue::Nil,
		}
	}
}

impl fmt::Display for InterpreterValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InterpreterValue::Function { .. } => write!(f, "function"),
			InterpreterValue::String(s) => write!(f, "{}", s),
			InterpreterValue::Number(n) => write!(f, "{}", n),
			InterpreterValue::False => write!(f, "false"),
			InterpreterValue::True => write!(f, "true"),
			InterpreterValue::Nil => write!(f, "nil"),
		}
	}
}

fn declare_native_functions(env: &WrappedInterpreterEnvironment) {
	env.declare(
		"str".to_string(),
		DeclaredValue {
			mutable: true,
			value: InterpreterValue::Function {
				fun: Rc::new(InterpreterFunction::Native {
					arity: 1,
					fun: |_env, args| {
						let input = &args[0];

						if let InterpreterValue::String(_) = input {
							Ok(input.clone())
						} else {
							Ok(InterpreterValue::String(Rc::from(
								args[0].to_string(),
							)))
						}
					},
				}),
				enclosing_env: env.clone(),
			},
		},
	);
}

// A shorthand way to extract identifier's name
pub fn assume_identifier(t: &Token) -> &str {
	if let TokenType::Identifier(i) = &t.token_type {
		i
	} else {
		unreachable!("Couldn't extract identifier. This shouldn't happen")
	}
}

pub fn guard_function(
	ibv: InterpreterStmtValue,
) -> Result<InterpreterValue, RuntimeError> {
	match ibv {
		InterpreterStmtValue::Break(token) => Err(RuntimeError {
			message: "Cannot use `break` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Continue(token) => Err(RuntimeError {
			message: "Cannot use `continue` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Return { value, .. } => Ok(value),
		InterpreterStmtValue::Noop => Ok(InterpreterValue::Nil),
	}
}

pub fn interpret(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let env = InterpreterEnvironment::new(None).wrap();

	declare_native_functions(&env);

	match evaluate_statements(statements, &env)? {
		InterpreterStmtValue::Noop => Ok(()),
		InterpreterStmtValue::Break(token) => Err(RuntimeError {
			message: "Cannot use `break` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Continue(token) => Err(RuntimeError {
			message: "Cannot use `continue` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Return { keyword, .. } => Err(RuntimeError {
			message: "Cannot use `return` outside of a function".into(),
			token: keyword,
		}),
	}
}

fn evaluate_statements(
	statements: &[Stmt],
	env: &WrappedInterpreterEnvironment,
) -> Result<InterpreterStmtValue, RuntimeError> {
	for stmt in statements.iter() {
		let e = evaluate(&stmt, env)?;

		if !matches!(e, InterpreterStmtValue::Noop) {
			return Ok(e);
		}
	}

	Ok(InterpreterStmtValue::Noop)
}

fn evaluate(
	stmt: &Stmt,
	env: &WrappedInterpreterEnvironment,
) -> Result<InterpreterStmtValue, RuntimeError> {
	match stmt {
		Stmt::Expression(v) => {
			eval_expression(&v.expression, env)?;

			Ok(InterpreterStmtValue::Noop)
		}
		Stmt::Print(v) => {
			let evaluated = eval_expression(&v.expression, env)?;

			println!("{}", evaluated);

			Ok(InterpreterStmtValue::Noop)
		}
		Stmt::Declaration(v) => {
			let value = v
				.initializer
				.as_ref()
				.map_or(Ok(InterpreterValue::Nil), |initializer| {
					eval_expression(&initializer, env)
				})?;

			env.declare(
				assume_identifier(&v.name).to_owned(),
				DeclaredValue {
					mutable: v.mutable,
					value,
				},
			);

			Ok(InterpreterStmtValue::Noop)
		}
		Stmt::Block(v) => {
			let new_scope = env.fork();

			evaluate_statements(&v.statements, &new_scope)
		}
		Stmt::If(v) => {
			if eval_expression(&v.condition, env)? == InterpreterValue::True {
				evaluate(&v.then, env)
			} else if let Some(otherwise) = &v.otherwise {
				evaluate(otherwise, env)
			} else {
				Ok(InterpreterStmtValue::Noop)
			}
		}
		Stmt::While(v) => {
			// these branches look so sketchy, but it's an optimization for
			// condition-less loops
			if let Some(condition) = &v.condition {
				while eval_expression(condition, env)? == InterpreterValue::True
				{
					let e = evaluate(&v.execute, env)?;

					match e {
						InterpreterStmtValue::Break(_) => break,
						InterpreterStmtValue::Continue(_) => continue,
						InterpreterStmtValue::Noop => (),
						InterpreterStmtValue::Return { .. } => {
							return Ok(e);
						}
					}
				}
			} else {
				loop {
					let e = evaluate(&v.execute, env)?;

					match e {
						InterpreterStmtValue::Break(_) => break,
						InterpreterStmtValue::Continue(_) => continue,
						InterpreterStmtValue::Noop => (),
						InterpreterStmtValue::Return { .. } => {
							return Ok(e);
						}
					}
				}
			}

			Ok(InterpreterStmtValue::Noop)
		}
		Stmt::Return(v) => Ok(InterpreterStmtValue::Return {
			value: v
				.expression
				.as_ref()
				.map_or(Ok(InterpreterValue::Nil), |e| {
					eval_expression(e, env)
				})?,
			keyword: v.keyword.clone(),
		}),
		Stmt::Break(v) => Ok(InterpreterStmtValue::Break(v.keyword.clone())),
		Stmt::Continue(v) => {
			Ok(InterpreterStmtValue::Continue(v.keyword.clone()))
		}
	}
}

fn eval_expression(
	expr: &Expr,
	env: &WrappedInterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Literal(v) => Ok(v.clone().into()),
		Expr::Grouping(v) => eval_expression(&v.expression, env),
		Expr::Unary(v) => eval_unary(v, env),
		Expr::Binary(v) => eval_binary(v, env),
		Expr::Identifier(v) => Ok(env.read(&v.name)?.value),
		Expr::Assignment(v) => {
			env.assign(&v.name, eval_expression(&v.value, env)?)
		}
		Expr::Call(v) => {
			fn confirm_arity(
				target: usize,
				value: usize,
				blame: &Token,
			) -> Result<(), RuntimeError> {
				if target != value {
					Err(RuntimeError {
						message: format!(
							"{} arguments",
							if value > target {
								"Too many"
							} else {
								"Not enough"
							}
						),
						token: blame.clone(),
					})
				} else {
					Ok(())
				}
			}

			fn map_arguments(
				parameters: &[Token],
				arguments: &[InterpreterValue],
				fun_env: &WrappedInterpreterEnvironment,
			) {
				parameters.iter().zip(arguments).for_each(|(param, arg)| {
					let name = assume_identifier(param);

					fun_env.declare(
						name.to_string(),
						DeclaredValue {
							mutable: true,
							value: arg.clone(),
						},
					);
				})
			}

			let callee = eval_expression(&v.calee, env)?;

			let (enclosing_env, fun) =
				if let InterpreterValue::Function { enclosing_env, fun } =
					callee
				{
					Ok((enclosing_env, fun))
				} else {
					Err(RuntimeError {
						message: format!(
							"Cannot call {}",
							callee.to_human_readable()
						),
						token: v.closing_paren.clone(),
					})
				}?;

			let arguments = v
				.arguments
				.iter()
				.map(|a| eval_expression(a, env))
				.collect::<Result<Vec<_>, RuntimeError>>()?;

			match &*fun {
				InterpreterFunction::LoxDefined(fv) => {
					confirm_arity(
						fv.params.as_ref().map_or(0, |p| p.len()),
						arguments.len(),
						&v.closing_paren,
					)?;

					let fun_env = &enclosing_env.fork();

					if let Some(params) = &fv.params {
						map_arguments(params, &arguments, fun_env)
					}

					if let Some(statements) = &fv.body {
						let e = evaluate_statements(&*statements, fun_env)?;
						Ok(guard_function(e)?)
					} else {
						Ok(InterpreterValue::Nil)
					}
				}
				InterpreterFunction::Native { arity, fun } => {
					confirm_arity(*arity, arguments.len(), &v.closing_paren)?;

					Ok(fun(&enclosing_env.fork(), &arguments)?)
				}
			}
		}
		Expr::Function(v) => {
			let fun = InterpreterValue::Function {
				enclosing_env: env.clone(),
				fun: Rc::new(InterpreterFunction::LoxDefined(FunctionValue {
					body: v.body.as_ref().map(|b| Rc::clone(b)),
					keyword: v.keyword.clone(),
					name: v.name.clone(),
					params: v.params.clone(),
				})),
			};

			if let Some(t) = &v.name {
				let iden = assume_identifier(t);

				env.declare(
					iden.to_string(),
					DeclaredValue {
						mutable: false,
						value: fun.clone(),
					},
				);
			}

			Ok(fun)
		}
	}
}

fn eval_unary(
	v: &UnaryValue,
	env: &WrappedInterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let right_value = eval_expression(&v.right, env)?;

	match (&v.operator.token_type, &right_value) {
		(TokenType::Minus, InterpreterValue::Number(n)) => {
			Ok(InterpreterValue::Number(-n))
		}
		(TokenType::Bang, InterpreterValue::True) => {
			Ok(InterpreterValue::False)
		}
		(TokenType::Bang, InterpreterValue::False) => {
			Ok(InterpreterValue::True)
		}

		_ => Err(RuntimeError {
			message: format!(
				"Cannot use `{}` on `{}`",
				v.operator.token_type, right_value
			),
			token: v.operator.clone(),
		}),
	}
}

fn eval_binary(
	v: &BinaryValue,
	env: &WrappedInterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	// first, match the logical operators, so that we can have short-circuiting
	match v.operator.token_type {
		TokenType::Or => {
			return Ok(
				if eval_expression(&v.left, env)? == InterpreterValue::True {
					InterpreterValue::True
				} else {
					eval_expression(&v.right, env)?
				},
			)
		}
		TokenType::And => {
			let left_value = eval_expression(&v.left, env)?;

			return Ok(if left_value == InterpreterValue::True {
				eval_expression(&v.right, env)?
			} else {
				left_value
			});
		}
		_ => (),
	}

	// then evaluate both sides normally
	let left_value = eval_expression(&v.left, env)?;
	let right_value = eval_expression(&v.right, env)?;

	// im sorry for this, but i found that the nested matches require
	// much simpler patterns,
	// and with this, i can achieve less comparisons overall
	match v.operator.token_type {
		TokenType::BangEqual => Ok((left_value != right_value).into()),
		TokenType::EqualEqual => Ok((left_value == right_value).into()),

		_ => match (&left_value, &right_value) {
			(InterpreterValue::Number(n1), InterpreterValue::Number(n2)) => {
				match v.operator.token_type {
					TokenType::Minus => Ok(InterpreterValue::Number(n1 - n2)),
					TokenType::Slash => Ok(InterpreterValue::Number(n1 / n2)),
					TokenType::Star => Ok(InterpreterValue::Number(n1 * n2)),
					TokenType::Plus => Ok(InterpreterValue::Number(n1 + n2)),
					TokenType::Greater => Ok((n1 > n2).into()),
					TokenType::GreaterEqual => Ok((n1 >= n2).into()),
					TokenType::Less => Ok((n1 < n2).into()),
					TokenType::LessEqual => Ok((n1 <= n2).into()),

					_ => unreachable!("Scanner did a bad job 😎."),
				}
			}
			(InterpreterValue::String(s1), InterpreterValue::String(s2)) => {
				if v.operator.token_type == TokenType::Plus {
					Ok(InterpreterValue::String(Rc::from(s1.to_string() + s2)))
				} else {
					Err(RuntimeError {
						message: format!(
							"You cannot use `{}` on two strings. Did you mean \
							 `+`?",
							v.operator.token_type
						),
						token: v.operator.clone(),
					})
				}
			}
			// error bby
			_ => Err(RuntimeError {
				message: format!(
					"Cannot use `{}` on {} and {}",
					v.operator.token_type,
					left_value.to_human_readable(),
					right_value.to_human_readable()
				),
				token: v.operator.clone(),
			}),
		},
	}
}
