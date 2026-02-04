PLATFORM ?= web
SERVER_URL ?= http://localhost:8080
BUNDLE_ARGS ?=
PACKAGES ?=
PACKAGE_FLAGS := $(foreach b,$(PACKAGES),--package-types $(b))
PACKAGE_NAME = roommates

ifdef VERBOSE
	BUNDLE_ARGS += --verbose --trace
endif

ifdef HOTPATCH
    HOT_PATCH_FLAG := --hot-patch
endif

ifeq ($(PLATFORM), android)
	BUNDLE_ARGS += --target aarch64-linux-android
endif

.PHONY: dependencies assets bundle bundle-no-deps bundle-android dev-server tests format clean
all: bundle


################### ASSETS ###################

FRONTEND_DIR := packages/frontend
DIST_DIR := dist
ASSETS_DIR := $(FRONTEND_DIR)/assets
GENERATED_ASSETS_DIR := $(ASSETS_DIR)/dist
APP_ICONS_DIR := $(GENERATED_ASSETS_DIR)/app_icons
APP_ICON_STAMP := $(APP_ICONS_DIR)/.stamp
ASSET_DEPS = $(GENERATED_ASSETS_DIR)/tailwind.css

ifneq ($(PLATFORM), web)
	ASSET_DEPS += $(APP_ICON_STAMP)
endif

NPM_DEPS = package.json package-lock.json
NODE_MODULES_STAMP = node_modules/.stamp

$(NODE_MODULES_STAMP): $(NPM_DEPS)
	npm ci
	touch $(NODE_MODULES_STAMP)


dependencies: $(NODE_MODULES_STAMP)

$(APP_ICON_STAMP): $(ASSETS_DIR)/roommatesicon.png dependencies
	npx @tauri-apps/cli icon $(ASSETS_DIR)/roommatesicon.png -o $(APP_ICONS_DIR)
	touch $(APP_ICON_STAMP)

$(GENERATED_ASSETS_DIR)/tailwind.css: $(FRONTEND_DIR)/tailwind.css dependencies
	npx @tailwindcss/cli -i $(FRONTEND_DIR)/tailwind.css -o $(GENERATED_ASSETS_DIR)/tailwind.css --minify --map

assets: $(ASSET_DEPS)


################# BUNDLING #################

KEYSTORE_PATH ?= $(HOME)/.android/keystore.jks
KEYSTORE_PASSWORD ?=
ANDROID_PROJECT_DIR := target/dx/$(PACKAGE_NAME)/release/android/app


bundle-android:
	@echo "Running custom Android bundle process"
	@KEYSTORE_PASSWORD=$(KEYSTORE_PASSWORD) ./scripts/bundle_android.sh $(ANDROID_PROJECT_DIR) $(DIST_DIR)/android $(APP_ICONS_DIR) $(KEYSTORE_PATH)

bundle-no-deps:
	SERVER_URL=$(SERVER_URL) dx bundle \
		--release \
		--package $(PACKAGE_NAME) \
		--$(PLATFORM) \
		--debug-symbols=false \
		--out-dir $(DIST_DIR)/$(PLATFORM) $(PACKAGE_FLAGS) $(BUNDLE_ARGS)

bundle: assets
	@echo "Building for platform: $(PLATFORM)"
ifeq ($(PLATFORM),android)
	rm -r $(ANDROID_PROJECT_DIR)/app/src/main/res/*
endif
	@set -e; \
	tmpfile=$$(mktemp); \
	mv $(FRONTEND_DIR)/tailwind.css $$tmpfile; \
	trap '[ -e $$tmpfile ] && mv $$tmpfile $(FRONTEND_DIR)/tailwind.css' INT TERM EXIT; \
	$(MAKE) bundle-no-deps
ifeq ($(PLATFORM),android)
	$(MAKE) bundle-android
endif


########################## UTILS ##########################

dev-server: dependencies
	dx serve --package $(PACKAGE_NAME) --platform $(PLATFORM) --addr 0.0.0.0 $(BUNDLE_ARGS) $(HOT_PATCH_FLAG)

tests: dependencies
	cargo test --workspace --all-features --no-fail-fast

format:
	@./scripts/format.sh

clean:
	rm -rf $(DIST_DIR)
	rm -rf $(GENERATED_ASSETS_DIR)
