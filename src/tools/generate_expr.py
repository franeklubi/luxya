from typing import List


def main():
	imports = [
		'crate::token',
	]

	literal_types = [
		'String	-> String',
		'Number	-> i32',
	]

	to_generate = [
		'Binary	-> left: Box<Expr>, operator: TokenType, right: Box<Expr>',
		'Grouping	-> expression: Box<Expr>',
		'Literal	-> value: LiteralValue',
		'Unary	-> operator: TokenType, right: Box<Expr>',
	]

	generate_ast("Expr", )


def generate_ast(
	base_name: str,
	types: List[str],
	imports: List[str],
	literal_types
):
	print('siema')


if __name__ == '__main__':
	main()
