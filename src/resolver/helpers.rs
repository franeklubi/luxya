// COMBAK: These are just temporary (I hope)

#[macro_export]
macro_rules! resolver_unwrap_scope {
	($wie:expr) => {{
		&$wie.env.borrow().scope
	}};
}

#[macro_export]
macro_rules! resolver_unwrap_scope_mut {
	($wie:expr) => {{
		&mut $wie.env.borrow_mut().scope
	}};
}

#[macro_export]
macro_rules! resolver_unwrap_enclosing {
	($wie:expr) => {{
		&$wie.env.borrow().enclosing
	}};
}
