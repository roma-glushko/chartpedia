
run: ## Run the CLI
	@cargo run

build: ## Build a binary
	@cargo build

lint: ## Lint the codebase
	@cargo fmt
	@cargo clippy

lint-fix: ## Fix lint issues if possible
	@cargo clippy --fix
