from typing import List, Tuple, Optional


ArrowExpr = Tuple[str, Optional[str]]

def generate_ast(
	base_name: str,
	types: List[str],
	imports: List[str],
	literal_types_name: Optional[str],
	literal_types: List[str],
) -> str:
	generated_file: str = ''

	# import everything
	for i in imports:
		generated_file += 'use {};\n'.format(i)

	generated_file += '\n'

	# generate literal values
	if literal_types_name != None:
		generated_file += '#[derive(Clone, PartialEq)]pub enum {} {{\n'.format(literal_types_name)

		for l in literal_types:
			lv = parse_arrow_expr(l)

			if lv is None:
				continue

			generated_file += '\t' + lv[0]

			if lv[1] != None:
				generated_file += '({})'.format(lv[1])

			generated_file += ',\n'

		generated_file += '}\n\n'

	enum_members: List[ArrowExpr] = []

	# generate value structs
	for t in types:
		member = parse_arrow_expr(t)

		if member is None:
			continue

		enum_members.append(member)

		if member[1] is None:
			continue

		# making fields public
		publicized: str = ', '.join(
			['pub {}'.format(field.strip()) for field in member[1].split(',')]
		)

		generated_file += 'pub struct {}Value {{{}}}\n\n'.format(member[0], publicized)

	# generate base enum
	generated_file += 'pub enum {} {{\n'.format(base_name)

	for member in enum_members:
		generated_file += '\t' + member[0]

		if member[1] != None:
			generated_file += '({}Value)'.format(member[0])

		generated_file += ',\n'

	generated_file += '}\n\n'

	return generated_file


# arrow expression is defined as such:
#      '<p1> -> <p2>' | '<p1>'
# the second parameter in the arrow expresssion is optionalk
def parse_arrow_expr(expr: str) -> Optional[ArrowExpr]:
	splout = expr.split('->')
	len_of_splout = len(splout)
	if len_of_splout < 1 or len_of_splout > 2:
		print('Invalid arrow expression: "{}"'.format(expr))
		return None

	p1: str = splout[0].strip()

	p2: Optional[str] = None

	if len_of_splout > 1:
		p2 = splout[1].strip()

	return (p1, p2)


def gen_expr() -> str:
	to_generate = [
		"""
			Function ->
				keyword: Token, name: Option<Token>,
				params: Option<Vec<Token>>, body: Option<Rc<Vec<Stmt>>>
		""",
		'Call -> calee: Box<Expr>, closing_paren: Token, arguments: Vec<Expr>',
		'Assignment -> name: Token, value: Box<Expr>, env_distance: Cell<u32>',
		'Binary -> left: Box<Expr>, operator: Token, right: Box<Expr>',
		'Get -> getee: Box<Expr>, property: Token, evaluate: bool',
		'Identifier -> name: Token, env_distance: Cell<u32>',
		'Unary -> operator: Token, right: Box<Expr>',
		'Grouping -> expression: Box<Expr>',
		'Literal(LiteralValue)',
	]

	imports = [
		'crate::{ast::stmt::*, token::Token}',
		'std::{rc::Rc, cell::Cell}',
	]

	literal_types = [
		'String -> Rc<str>',
		'Number -> f64',
		'True',
		'False',
		'Nil',
	]

	return generate_ast(
		'Expr',
		to_generate,
		imports,
		'LiteralValue',
		literal_types,
	)


def gen_stmt() -> str:
	to_generate = [
		"""
			For ->
				condition: Option<Box<Expr>>, body: Box<Stmt>,
				closer: Option<Box<Stmt>>
		""",
		"""
			If ->
				condition: Box<Expr>, then: Box<Stmt>,
				otherwise: Option<Box<Stmt>>
		""",
		"""
			Declaration ->
				name: Token, initializer: Option<Box<Expr>>,
				mutable: bool
		""",
		'Return -> keyword: Token, expression: Option<Expr>',
		'Class -> name: Token, methods: Vec<Expr>',
		'Expression -> expression: Box<Expr>',
		'Block -> statements: Vec<Stmt>',
		'Print -> expression: Box<Expr>',
		'Continue -> keyword: Token',
		'Break -> keyword: Token',
	]

	imports = [
		'crate::token::Token',
		'crate::ast::expr::Expr',
	]

	literal_types: List[str] = []

	return generate_ast(
		'Stmt',
		to_generate,
		imports,
		None,
		literal_types,
	)

def write_to_file(text: str, path: str) -> None:
	with open(path, 'w') as f:
		f.write(text)


def main() -> None:
	write_to_file(gen_expr(), './src/ast/expr.rs')
	write_to_file(gen_stmt(), './src/ast/stmt.rs')


if __name__ == '__main__':
	main()
