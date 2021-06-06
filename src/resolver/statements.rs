use super::{env::*, resolve};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
	interpreter::{
		helpers::assume_identifier,
		types::{InterpreterValue, RuntimeError, StmtResult},
	},
};


#[inline(always)]
pub fn print_statement(
	v: &PrintValue,
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	resolve::expression(&v.expression, env)?;

	Ok(StmtResult::Noop)
}

pub fn declaration_statement(
	v: &DeclarationValue,
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	if let Some(i) = &v.initializer {
		resolve::expression(i, env)?;
	}

	env.declare(
		assume_identifier(&v.name).to_owned(),
		DeclaredValue {
			mutable: v.mutable,
			value: InterpreterValue::Nil,
		},
	);

	Ok(StmtResult::Noop)
}

pub fn if_statement(
	v: &IfValue,
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	resolve::expression(&v.condition, env)?;

	if let Some(then) = &v.then {
		resolve::statement(then, env)?;
	}

	if let Some(otherwise) = &v.otherwise {
		resolve::statement(otherwise, env)?;
	}

	Ok(StmtResult::Noop)
}

pub fn for_statement(
	v: &ForValue,
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	resolve::statement(&v.body, env)?;

	if let Some(condition) = &v.condition {
		resolve::expression(condition, env)?;
	}

	if let Some(closer) = &v.closer {
		resolve::statement(closer, env)?;
	}

	Ok(StmtResult::Noop)
}

pub fn class_statement(
	v: &ClassValue,
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
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

		resolve::expression(expr, env)?;

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
		// expression wires the method to function_expression
		resolve::expression(method, &class_env)?;
	}

	Ok(StmtResult::Noop)
}
