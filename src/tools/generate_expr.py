from typing import List


def generate_ast(
	base_name: str,
	types: List[str],
	imports: List[str],
	literal_types: List[str],
) -> None:
	print('siema')


def main() -> None:
	to_generate = [
		'Binary	-> left: Box<Expr>, operator: TokenType, right: Box<Expr>',
		'Grouping	-> expression: Box<Expr>',
		'Literal	-> value: LiteralValue',
		'Unary	-> operator: TokenType, right: Box<Expr>',
	]

	imports = [
		'crate::token',
	]

	literal_types = [
		'String	-> String',
		'Number	-> i32',
	]

	generate_ast("Expr", to_generate, imports, literal_types);


if __name__ == '__main__':
	main()
