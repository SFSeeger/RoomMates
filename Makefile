PLATFORM ?= web

FRONTEND_DIR := packages/frontend
DIST_DIR := dist
GENERATED_ASSETS_DIR := $(FRONTEND_DIR)/assets/dist

NPM_DEPS = package.json package-lock.json
NODE_MODULES_STAMP = node_modules/.stamp

.PHONY: dependencies bundle dev-server tests format clean

all: bundle

$(NODE_MODULES_STAMP): $(NPM_DEPS)
	npm ci
	touch $(NODE_MODULES_STAMP)

dependencies: $(NODE_MODULES_STAMP)

bundle: dependencies
	@echo "Building for platform: $(PLATFORM)"
	npx @tailwindcss/cli -i $(FRONTEND_DIR)/tailwind.css -o $(GENERATED_ASSETS_DIR)/tailwind.css --minify --map
	dx bundle --package frontend --release --platform $(PLATFORM) --debug-symbols=false --out-dir $(DIST_DIR)/$(PLATFORM)

dev-server: dependencies
	dx serve --package frontend --platform $(PLATFORM) --addr 0.0.0.0

tests: dependencies
	cargo test --workspace --all-features --no-fail-fast

format:
	@./scripts/format.sh

clean:
	rm -rf $(DIST_DIR)
	rm -rf $(GENERATED_ASSETS_DIR)
