
ifndef VERBOSE
.SILENT:
endif

sample_program_path=./src/sample_program.lox
generate_ast_path=./tools/generate_ast.py

main:
	cargo build

prod:
	make -s clippy
	clear
	cargo build --release

clippy:
	clear
	cargo clippy --all-features -- -D warnings

run:
	make -s clippy
	clear
	cargo run

sample:
	make -s clippy
	clear
	cargo run -- ${sample_program_path}

fmt:
	cargo fmt

watch:
	cargo watch -x "fmt; make -s run"

watch_sample:
	cargo watch -x "fmt; make -s sample"

generate: ${generate_ast_path}
	python3 ${generate_ast_path}

generate_check: ${generate_ast_path}
	mypy --check-untyped-defs ${generate_ast_path} && make -s generate

watch_generate: ${generate_ast_path}
	echo ${generate_ast_path} | entr -c make -s generate_check
