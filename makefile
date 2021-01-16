
ifndef VERBOSE
.SILENT:
endif

sample_program_path=./src/sample_program.lox
generate_expr_path=./src/tools/generate_expr.py

main:
	cargo build

run:
	cargo run

sample:
	cargo run -- ${sample_program_path}

fmt:
	cargo fmt

watch:
	cargo watch -x "fmt; make -si sample"

generate: ${generate_expr_path}
	mypy ${generate_expr_path} && python3 ${generate_expr_path}

watch_generate: ${generate_expr_path}
	echo ${generate_expr_path} | entr make -si generate
