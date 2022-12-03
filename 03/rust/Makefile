.SILENT:

.DEFAULT_GOAL := help

help: ## Shows this help page
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: build
build:
	cargo build

.PHONY: build_optimised
build_optimised:
	cargo build --release

.PHONY: part_a_example
part_a_example: build ## Solve part A with example input
	target/debug/solution --solve-a ../exampleInput.txt

.PHONY: part_a
part_a: build_optimised ## Solve part A with real input
	target/release/solution --solve-a ../input.txt

.PHONY: part_b_example
part_b_example: build ## Solve part B with example input
	target/debug/solution --solve-b ../exampleInput.txt

.PHONY: part_b
part_b: build_optimised ## Solve part B with real input
	target/release/solution --solve-b ../input.txt

.PHONY: time
time: build_optimised
	AOC_EXAMPLE_MODE=0 time target/release/solution --solve-a ../input.txt
	AOC_EXAMPLE_MODE=0 time target/release/solution --solve-b ../input.txt
