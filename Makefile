HEADER = inst/usr/include/gobject-example-0.1/ex.h
GIR = inst/usr/share/gir-1.0/Ex-0.1.gir
TYPELIB = inst/usr/lib64/girepository-1.0/Ex-0.1.typelib
VAPI = inst/usr/share/vala/vapi/gobject-example-0.1.vapi

RUST_SOURCES = \
	src/lib.rs \
	src/color/ffi.rs \
	src/color/imp.rs \
	src/color/mod.rs \
	src/error/ffi.rs \
	src/error/imp.rs \
	src/error/mod.rs \
	src/flags/ffi.rs \
	src/flags/imp.rs \
	src/flags/mod.rs \
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

all: $(GIR) $(TYPELIB) $(VAPI)

export PKG_CONFIG_PATH=$(PWD)/inst/usr/lib64/pkgconfig
export GI_TYPELIB_PATH=$(PWD)/inst/usr/lib64/girepository-1.0
export LD_LIBRARY_PATH=$(PWD)/inst/usr/lib64

$(HEADER): $(RUST_SOURCES)
	cargo cinstall --release --destdir=inst --prefix=/usr --libdir=/usr/lib64

$(GIR): $(HEADER)
	mkdir -p $(@D)
	g-ir-scanner -v --warn-all \
		--namespace Ex --nsversion=0.1 \
		-Iinst/include --c-include "ex.h" \
		--library=gobject_example --library-path=inst/usr/lib64 \
		--include=GObject-2.0 -pkg gobject-2.0 \
		--output $@ \
		$<

$(TYPELIB): $(GIR)
	mkdir -p $(@D)
	g-ir-compiler $< -o $@

$(VAPI): $(GIR)
	mkdir -p $(@D)
	vapigen \
		--library gobject-example-0.1 \
		$< -d $(@D)

run-python: $(TYPELIB)
	python3 test.py

run-gjs: $(TYPELIB)
	gjs test.js

test-vala: test.vala $(VAPI)
	valac -v \
		--vapidir=inst/usr/share/vala/vapi \
		-X -Iinst/usr/include/gobject-example-0.1 \
		-X -Linst/usr/lib64 \
		--pkg=gobject-example-0.1 \
		$< -o $@

run-vala: test-vala
	./test-vala

test-c: test.c $(HEADER)
	$(CC) -Wall \
		$(shell pkg-config --cflags --libs gobject-example-0.1) \
		-Iinst/usr/include/gobject-example-0.1 \
		-Linst/usr/lib64 \
		$< -o $@

run-c: test-c
	./test-c

check:
	cargo test

check-bindings: $(HEADER)
	cargo test --features=bindings

install: $(HEADER)
	sudo cp -r inst/* $(DESTDIR)/

check-all: check check-bindings run-c run-python run-gjs run-vala
