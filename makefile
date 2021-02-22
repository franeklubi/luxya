
ifndef VERBOSE
.SILENT:
endif

sample_program_path=./src/sample_program.lox
generate_expr_path=./tools/generate_expr.py

main:
	cargo build

clippy:
	cargo clippy --all-features -- -D warnings

run:
	make -s clippy
	cargo run

sample:
	make -s clippy
	cargo run -- ${sample_program_path}

fmt:
	cargo fmt

watch:
	cargo watch -x "fmt; clear; make -s run"

watch_sample:
	cargo watch -x "fmt; clear; make -s sample"

generate: ${generate_expr_path}
	python3 ${generate_expr_path}

generate_check: ${generate_expr_path}
	mypy --check-untyped-defs ${generate_expr_path} && make -s generate

watch_generate: ${generate_expr_path}
	echo ${generate_expr_path} | entr -c make -s generate_check
