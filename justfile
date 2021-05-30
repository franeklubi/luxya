
sample_program_path := './src/sample_program.lux'
generate_ast_path := './tools/generate_ast.py'

main:
	just clippy
	clear
	cargo build --release --verbose

release:
	cargo build --release --verbose

clippy:
	clear
	cargo clippy --all-features -- -D warnings

run:
	just clippy
	clear
	cargo run

sample:
	echo siema
	just clippy
	clear
	cargo run -- {{sample_program_path}}

watch:
	cargo watch -x "fmt; just run"

watch_sample:
	cargo watch -x "fmt; just sample"

generate_ast:
	python3 {{generate_ast_path}}

generate_ast_check:
	mypy --check-untyped-defs {{generate_ast_path}} && just generate_ast

watch_generate_ast:
	echo {{generate_ast_path}} | entr -c just generate_ast_check
