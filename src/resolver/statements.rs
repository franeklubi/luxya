use super::{resolve, resolver_env::*};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
	interpreter::{
		helpers::assume_identifier,
		types::{InterpreterStmtValue, InterpreterValue, RuntimeError},
	},
};


#[inline(always)]
pub fn print_statement(
	v: &PrintValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	resolve::resolve_expression(&v.expression, env)?;

	Ok(InterpreterStmtValue::Noop)
}

pub fn declaration_statement(
	v: &DeclarationValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	if let Some(i) = &v.initializer {
		resolve::resolve_expression(i, env)?;
	}

	env.declare(
		assume_identifier(&v.name).to_owned(),
		DeclaredValue {
			mutable: v.mutable,
			value: InterpreterValue::Nil,
		},
	);

	Ok(InterpreterStmtValue::Noop)
}

pub fn if_statement(
	v: &IfValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	resolve::resolve_expression(&v.condition, env)?;

	if let Some(then) = &v.then {
		resolve::resolve_statement(then, env)?;
	}

	if let Some(otherwise) = &v.otherwise {
		resolve::resolve_statement(otherwise, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

pub fn for_statement(
	v: &ForValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	resolve::resolve_statement(&v.body, env)?;

	if let Some(condition) = &v.condition {
		resolve::resolve_expression(condition, env)?;
	}

	if let Some(closer) = &v.closer {
		resolve::resolve_statement(closer, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

pub fn class_statement(
	v: &ClassValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	let iden = assume_identifier(&v.name);

	env.declare(
		iden.to_owned(),
		DeclaredValue {
			mutable: false,
			value: InterpreterValue::Nil,
		},
	);

	let superclass_env = if let Some(expr) = &v.superclass {
		let superclass = if let Expr::Identifier(s) = expr {
			s
		} else {
			unreachable!("Superclass should be an identifier expression")
		};

		let super_iden = assume_identifier(&superclass.name);

		if super_iden == iden {
			return Err(RuntimeError {
				message: "Class cannot inherit from itself".into(),
				token: superclass.name.clone(),
			});
		}

		resolve::resolve_expression(expr, env)?;

		let superclass_env = env.fork();

		superclass_env.declare(
			"super".to_owned(),
			DeclaredValue {
				mutable: false,
				value: InterpreterValue::Nil,
			},
		);

		Some(superclass_env)
	} else {
		None
	};

	let class_env = superclass_env.map_or_else(|| env.fork(), |dce| dce.fork());

	class_env.declare(
		"this".to_owned(),
		DeclaredValue {
			mutable: false,
			value: InterpreterValue::Nil,
		},
	);

	for method in &v.methods {
		// resolve_expression wires the method to function_expression
		resolve::resolve_expression(method, &class_env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}
