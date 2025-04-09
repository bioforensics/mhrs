## #===== development tasks =====#
##

## help:        print this help message and exit
help: Makefile
	@sed -n 's/^## //p' Makefile

## test:        run test suite
test:
	cargo test

build:
	cargo llvm-cov --ignore-filename-regex 'src/(main|cli).rs' --no-report

## testcov:     run test suite and show test coverage
testcov:
	cargo llvm-cov --show-missing-lines --ignore-filename-regex 'src/(main|cli).rs'

## style:       check code style
style:
	cargo fmt --check

## format:      autoformat code
format:
	cargo fmt

## doc:         build documentation
doc:
	cargo doc --no-deps

## loc:         count lines of code
loc:
	cargo warloc

## release:     compile release binary
release:
	cargo build --release

## hooks:       deploy git pre-commit hooks for development
hooks:
	echo "set -eo pipefail" > .git/hooks/pre-commit
	echo "make style" >> .git/hooks/pre-commit
	echo "make doc" >> .git/hooks/pre-commit
	chmod 755 .git/hooks/pre-commit
