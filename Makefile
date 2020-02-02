PROJ_DIR := $(realpath .)
BUILD_DIR := $(PROJ_DIR)/build

MODS := core
BUILD_MODS := $(patsubst %,%.build,$(MODS))
TEST_MODS  := $(patsubst %,%.test,$(MODS))

LIBS_DIR := libs
BUILD_LIBS := $(LIBS_DIR).build
TEST_LIBS  := $(LIBS_DIR).test


.PHONY: all
all: build


.PHONY: build-dir
build-dir:
	@mkdir -p $(BUILD_DIR)


.PHONY: build
build: $(BUILD_LIBS) $(BUILD_MODS)

.PHONY: test
test: $(TEST_LIBS) $(TEST_MODS)

.PHONY: $(BUILD_MODS)
$(BUILD_MODS): %.build:% build-dir
	$(MAKE) -C $< build BUILD_DIR=$(BUILD_DIR)

.PHONY: $(TEST_MODS)
$(TEST_MODS): %.test:% build-dir
	$(MAKE) -C $< test BUILD_DIR=$(BUILD_DIR)

.PHONY: $(BUILD_LIBS)
$(BUILD_LIBS): %.build:% build-dir
	$(MAKE) -C $< BUILD_DIR=$(BUILD_DIR)

.PHONY: $(TEST_LIBS)
$(TEST_LIBS): %.test:% build-dir
	$(MAKE) -C $< test BUILD_DIR=$(BUILD_DIR)


.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
