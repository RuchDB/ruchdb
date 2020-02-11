MOD ?= default

ifdef BUILD_DIR
	MOD_BUILD_DIR := $(realpath $(BUILD_DIR))/$(MOD)/target
else
	MOD_BUILD_DIR := $(realpath .)/target
endif

RCC   := cargo build --release
RTEST := cargo test
RF := cargo fmt
RFC := cargo fmt -- --check


.PHONY: all
all: $(MOD)


.PHONY: build-dir
build-dir:
	@mkdir -p $(MOD_BUILD_DIR)


.PHONY: $(MOD)
$(MOD): build

.PHONY: build
build: build-dir
	$(RCC) --target-dir $(MOD_BUILD_DIR)

.PHONY: fmt
fmt:
	$(RF)

.PHONY: fmt-check
fmt-check:
	$(RFC)

.PHONY: test
test: build-dir
	$(RTEST) --target-dir $(MOD_BUILD_DIR)


.PHONY: clean
clean:
	rm -rf $(MOD_BUILD_DIR)
