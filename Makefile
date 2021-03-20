HEADERS = \
	include/ex/ex.h \
	include/ex/color.h \
	include/ex/foo.h \
	include/ex/bar.h \
	include/ex/nameable.h \
	include/ex/rstring.h \
	include/ex/shared-rstring.h \

RUST_SOURCES = \
	src/lib.rs \
	src/color/ffi.rs \
	src/color/imp.rs \
	src/color/mod.rs \
	src/foo/ffi.rs \
	src/foo/imp.rs \
	src/foo/mod.rs \
	src/bar/ffi.rs \
	src/bar/imp.rs \
	src/bar/mod.rs \
	src/nameable/ffi.rs \
	src/nameable/imp.rs \
	src/nameable/mod.rs \
	src/rstring/ffi.rs \
	src/rstring/imp.rs \
	src/rstring/mod.rs \
	src/shared_rstring/ffi.rs \
	src/shared_rstring/imp.rs \
	src/shared_rstring/mod.rs

all: Ex-0.1.gir Ex-0.1.typelib Ex-0.1.vapi

export PKG_CONFIG_PATH=$(PWD)
export GI_TYPELIB_PATH=$(PWD)
export LD_LIBRARY_PATH=$(PWD)/target/debug

target/debug/libgobject_example.so: $(RUST_SOURCES)
	cargo build

Ex-0.1.gir: target/debug/libgobject_example.so $(HEADERS)
	g-ir-scanner -v --warn-all \
		--namespace Ex --nsversion=0.1 \
		-Iinclude --c-include "ex/ex.h" \
		--library=gobject_example --library-path=target/debug \
		--include=GObject-2.0 -pkg gobject-2.0 \
		--output $@ \
		$(HEADERS)

Ex-0.1.typelib: Ex-0.1.gir
	g-ir-compiler \
		--includedir=include \
		$< -o $@

Ex-0.1.vapi: Ex-0.1.gir
	vapigen \
		--library Ex-0.1 \
		$<

clean:
	rm -f Ex-0.1.typelib
	rm -f Ex-0.1.gir
	rm -f Ex-0.1.vapi test-vala
	rm -rf test-c
	cargo clean

run-python: Ex-0.1.typelib
	python3 test.py

run-gjs: Ex-0.1.typelib
	gjs test.js

test-vala: test.vala Ex-0.1.vapi
	valac -v \
		--vapidir=$(PWD) \
		--pkg=Ex-0.1 \
		$< -o $@

run-vala: test-vala
	./test-vala

test-c: test.c target/debug/libgobject_example.so $(HEADERS)
	$(CC) `pkg-config --cflags --libs Ex-0.1` $< -o $@

run-c: test-c
	./test-c

check:
	cargo test
