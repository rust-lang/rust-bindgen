
TEST_HEADERS := $(wildcard tests/headers/*.h) $(wildcard tests/headers/*.hpp)

TEST_TARGETS := $(TEST_HEADERS:.h=.rs)
TEST_TARGETS := $(TEST_TARGETS:.hpp=.rs)
TEST_TARGETS := $(patsubst tests/headers/%, tests/expectations/tests/%, $(TEST_TARGETS))

BINDGEN := ./target/debug/bindgen

.PHONY: $(BINDGEN)
$(BINDGEN):
	[ -f $@ ] || cargo build --features llvm_stable

.PHONY: test
test:
	cargo test --features llvm_stable


.PHONY: regen-tests
regen-tests: $(BINDGEN) clean-tests $(TEST_TARGETS)
	@echo > /dev/null

.PHONY: clean-tests
clean-tests:
	$(RM) $(TEST_TARGETS)

# TODO: Add options to add flags and whatnot
tests/expectations/tests/%.rs: tests/headers/%.h
	@mkdir -p $(dir $@)
	./tests/tools/run-bindgen.py $(BINDGEN) $< $@

tests/expectations/tests/%.rs: tests/headers/%.hpp
	@mkdir -p $(dir $@)
	./tests/tools/run-bindgen.py $(BINDGEN) $< $@
