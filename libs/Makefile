ifdef BUILD_DIR
	LIB_BUILD_DIR := $(realpath $(BUILD_DIR))/libs
else
	LIB_BUILD_DIR := $(realpath .)/build
endif

MODS := $(filter-out Makefile build,$(shell ls))


.PHONY: all
all: $(MODS)


.PHONY: build-dir
build-dir:
	@mkdir -p $(LIB_BUILD_DIR)


.PHONY: $(MODS)
$(MODS): build-dir
	$(MAKE) -C $@ BUILD_DIR=$(LIB_BUILD_DIR)

.PHONY: fmt
fmt:
	for mod in $(MODS); do \
  		$(MAKE) -C $$mod $@; \
	done

.PHONY: fmt-check
fmt-check:
	for mod in $(MODS); do \
  		$(MAKE) -C $$mod $@; \
	done

.PHONY: test
test: build-dir
	for mod in $(MODS); do \
		$(MAKE) -C $$mod $@ BUILD_DIR=$(LIB_BUILD_DIR); \
	done


.PHONY: clean
clean:
	rm -rf $(LIB_BUILD_DIR)
