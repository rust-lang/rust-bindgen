
TEST_HEADERS := $(wildcard tests/headers/*.h) $(wildcard tests/headers/*.hpp)

TEST_TARGETS := $(TEST_HEADERS:.h=.rs)
TEST_TARGETS := $(TEST_TARGETS:.hpp=.rs)
TEST_TARGETS := $(patsubst tests/headers/%, tests/expectations/%, $(TEST_TARGETS))

BINDGEN := ./target/debug/bindgen

.PHONY: $(BINDGEN)
$(BINDGEN):
	[ -f $@ ] || cargo build

.PHONY: test
test: regen-tests
	@echo > /dev/null


.PHONY: regen-tests
regen-tests: $(BINDGEN) clean-tests $(TEST_TARGETS)
	@echo > /dev/null

.PHONY: clean-tests
clean-tests:
	$(RM) $(TEST_TARGETS)

# TODO: Add options to add flags and whatnot
tests/expectations/%.rs: tests/headers/%.h
	@mkdir -p $(dir $@)
	./tests/tools/run-bindgen.py $(BINDGEN) $< $@

tests/expectations/%.rs: tests/headers/%.hpp
	@mkdir -p $(dir $@)
	./tests/tools/run-bindgen.py $(BINDGEN) $< $@
