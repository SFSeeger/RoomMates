PLATFORM ?= web

ifdef HOTPATCH
    HOT_PATCH_FLAG := --hot-patch
endif

NPM_DEPS = package.json package-lock.json
NODE_MODULES_STAMP = node_modules/.stamp

$(NODE_MODULES_STAMP): $(NPM_DEPS)
	npm ci
	touch $(NODE_MODULES_STAMP)

dependencies: $(NODE_MODULES_STAMP)

dev-server: dependencies
	dx serve --package frontend --platform $(PLATFORM) --addr 0.0.0.0  $(HOT_PATCH_FLAG)

tests: dependencies
	cargo test --workspace --all-features --no-fail-fast

format:
	@./scripts/format.sh

clean:
	rm -rf node_modules
	rm -rf target
	rm -rf packages/frontend/dist
