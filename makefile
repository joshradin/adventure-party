MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables
.PRECIOUS: %.wasm

TOOLCHAIN:=stable
CARGO:=cargo +$(TOOLCHAIN)
TARGET_DIR:=target
DIST_DIR:=$(TARGET_DIR)/dist
ifeq ($(PROFILE), release)
FLAG:=--release
WASM_PACK_FLAG:=--release
else
PROFILE:=debug
FLAG:=
WASM_PACK_FLAG:=--dev
endif

WASM_PACKAGES=game_client ssr_hydrate
BINARY_PACKAGES:=game_server site

WASM_TARGET=wasm32-unknown-unknown

.PHONY: default
default: build



.PHONY: debug
debug:
	@echo ALL_PACKAGES=$(ALL_PACKAGES)
	@echo WASM_PACKAGES=$(WASM_PACKAGES)
	@echo NATIVE_PACKAGES=$(NATIVE_PACKAGES)


.PHONY: $(WASM_PACKAGES) $(BINARY_PACKAGES) $(addprefix $(TARGET_DIR)/wasm-pack/$(PROFILE),$(WASM_PACKAGES))

$(TARGET_DIR)/$(PROFILE)/%: $(wildcard crates/*/src/**.rs) $(wildcard crates/*/templates/**.html) crates/%/Cargo.toml Cargo.lock
	$(CARGO) build $(FLAG) --package $*
	@touch $@

$(TARGET_DIR)/$(WASM_TARGET)/$(PROFILE)/%.wasm: crates/*/src/**.rs crates/%/Cargo.toml Cargo.lock
	$(CARGO) build $(FLAG) --package $* --target $(WASM_TARGET) --lib
	@touch $@

$(BINARY_PACKAGES): %: $(TARGET_DIR)/$(PROFILE)/%

$(TARGET_DIR)/wasm-pack/$(PROFILE)/%: $(TARGET_DIR)/$(WASM_TARGET)/$(PROFILE)/%.wasm
	wasm-pack build -t web $(WASM_PACK_FLAG) crates/$* -d ../../$@

$(WASM_PACKAGES): %: $(TARGET_DIR)/wasm-pack/$(PROFILE)/%

.PHONY: serve build dist
build: $(WASM_PACKAGES) $(BINARY_PACKAGES)

target/dist:
	@mkdir -p $(DIST_DIR)

$(DIST_DIR)/public: $(DIST_DIR)
	@mkdir -p $(DIST_DIR)/public

$(DIST_DIR)/static/%: static/% | $(DIST_DIR)
	@mkdir -p $(DIST_DIR)/static
	cp -r $< $(dir $@)

$(DIST_DIR)/public/%: public/% | $(DIST_DIR) $(DIST_DIR)/public
	@mkdir -p $(DIST_DIR)/public
	cp -r $< $(dir $@)

.PHONY: clean-dist
clean-dist:
	@rm -rf $(DIST_DIR)

.PHONY: clean
clean: | clean-dist
	@test -d target && $(CARGO) clean || true

target/dist/public/%: $(TARGET_DIR)/wasm-pack/$(PROFILE)/% | $(DIST_DIR)/public
	@#echo Copying $< to $(dir $@)
	cp -r $< $(dir $@)

copy-wasms: $(addprefix target/dist/public/,$(WASM_PACKAGES))

dist: build $(addprefix target/dist/,$(wildcard static/*)) $(addprefix target/dist/,$(wildcard public/*)) copy-wasms

serve: build dist
	$(CARGO) run $(FLAG) --bin site -- --cwd=$(DIST_DIR) --debug
