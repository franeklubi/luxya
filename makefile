
ifndef VERBOSE
.SILENT:
endif

sample_program_path=./src/sample_program.lux
generate_ast_path=./tools/generate_ast.py

release:
	make -s generate_ast_check
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

watch:
	cargo watch -x "fmt; make -s run"

watch_sample:
	cargo watch -x "fmt; make -s sample"

generate_ast: ${generate_ast_path}
	python3 ${generate_ast_path}

generate_ast_check: ${generate_ast_path}
	mypy --check-untyped-defs ${generate_ast_path} && make -s generate_ast

watch_generate_ast: ${generate_ast_path}
	echo ${generate_ast_path} | entr -c make -s generate_ast_check
