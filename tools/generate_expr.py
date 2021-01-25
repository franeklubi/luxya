from typing import List, Tuple, Optional
from sys import exit


ArrowExpr = Tuple[str, Optional[str]]

def generate_ast(
	base_name: str,
	types: List[str],
	imports: List[str],
	literal_types_name: str,
	literal_types: List[str],
) -> str:
	generated_file: str = ''

	# import everything
	for i in imports:
		generated_file += 'use {};\n'.format(i)

	generated_file += '\n'

	# generate literal values
	generated_file += 'pub enum {} {{\n'.format(literal_types_name)

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

		generated_file += 'pub struct {}Value {{{}}}\n\n'.format(member[0], member[1])

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


def main() -> None:
	to_generate = [
		'Binary	-> left: Box<Expr>, operator: TokenType, right: Box<Expr>',
		'Grouping	-> expression: Box<Expr>',
		'Literal(LiteralValue)',
		'Unary	-> operator: TokenType, right: Box<Expr>',
	]

	imports = [
		'crate::token::TokenType',
	]

	literal_types = [
		'String	-> String',
		'Number	-> f64',
		'True',
		'False',
		'Nil',
	]

	generated = generate_ast(
		'Expr',
		to_generate,
		imports,
		'LiteralValue',
		literal_types,
	)

	print(generated, end='')


if __name__ == '__main__':
	main()
