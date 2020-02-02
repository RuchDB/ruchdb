MOD := rtypes

ifdef BUILD_DIR
	MOD_BUILD_DIR := $(realpath $(BUILD_DIR))/$(MOD)/target
else
	MOD_BUILD_DIR := $(realpath .)/target
endif

RCC   := cargo build --release
RTEST := cargo test


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

.PHONY: test
test: build-dir
	$(RTEST) --target-dir $(MOD_BUILD_DIR)


.PHONY: clean
clean:
	rm -rf $(MOD_BUILD_DIR)
