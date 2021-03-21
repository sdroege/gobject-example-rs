# Example for exporting a GObject/C API from Rust

This repository contains an example [Rust](https://www.rust-lang.org/) crate
that can compile to a C-compatible shared library and that exports a
[GObject](https://developer.gnome.org/gobject/stable) C API.

At the same time the API provided by this crate can be used directly from Rust
by statically linking this crate as well as by dynamically linking against the
C-compatible library. Both variants provide exactly the same Rust API.

In addition it provides a way to generate [GObject-Introspection](https://gitlab.gnome.org/GNOME/gobject-introspection/)
metadata that allows usage from other languages, like [Python](https://gitlab.gnome.org/GNOME/pygobject/) and
[JavaScript](https://gitlab.gnome.org/GNOME/gjs).

## Implemented Example APIs

### General Structure

Each type comes with 3 Rust modules

  * `mod.rs`: Contains a Rust wrapper of the API. This is basically the same
    as what [gir](https://github.com/gtk-rs/gir) would autogenerate, and
    follows the same patterns as the GLib, GTK, etc. bindings.
  * `imp.rs`: Contains the definition of the types together with the actual
    private implementation, plus an inline FFI module that exports
    C-compatible FFI functions.
  * `ffi.rs`: Contains Rust FFI definitions of the exported C types. This is
    only used in combination with the `bindings` cargo feature (see below).

and a C header that is (for now, see [issue 6](https://github.com/sdroege/gobject-example-rs/pull/6))
manually written.

### Details

Pending refactoring in [issue 10](https://github.com/sdroege/gobject-example-rs/issues/10).

## Usage

### Usage from Rust

The API from the `mod.rs` can directly be used from Rust code and follows the
same patterns as the GLib, GTK, etc. bindings.

There is example usage in the inline tests inside the `mod.rs` of each type.

#### Statically linked crate

The crate can be directly added as a dependency in some other projects
`Cargo.toml` and then the API from the individual `mod.rs` is available.

This statically links the implementation into the application.

Running `make check` would run tests in this mode.

#### Dynamically linked C library (`bindings` feature)

When adding the crate as a dependency and enabling the `bindings` cargo
feature then the actual implementation of all the types is omitted. Instead
dynamic linking against the implementation from the C-compatible shared
library will happen.

The API is otherwise exactly the same.

Running `make check-bindings` would run tests in this mode.

### Usage from C

Running `cargo build` will create a C-compatible shared library in
`target/debug/libgobject_example.so`. The corresponding headers for the API
can be found in the `include` directory.

`test.c` contains some example usage of the API and `make run-c` compiles and
runs this.

### Usage from Python

Via [gobject-introspection](https://gitlab.gnome.org/GNOME/gobject-introspection/)
a `Ex-0.1.typelib` file is created. This can be used by [pygobject](https://gitlab.gnome.org/GNOME/pygobject/)
to expose the API directly to Python.

`test.py` contains some example usage of the API and `make run-python` runs this.

### Usage from JavaScript/GJS

Via [gobject-introspection](https://gitlab.gnome.org/GNOME/gobject-introspection/)
a `Ex-0.1.typelib` file is created. This can be used by [gjs](https://gitlab.gnome.org/GNOME/gjs)
to expose the API directly to JavaScript. An alternative for [node.js](https://nodejs.org) would
be [node-gtk](https://github.com/romgrk/node-gtk).

`test.js` contains some example usage of the API and `make run-gjs` runs this.

### Usage from Vala

Via [gobject-introspection](https://gitlab.gnome.org/GNOME/gobject-introspection/)
a `Ex-0.1.gir` file is created that contains an XML representation of the API.
[Vala](https://wiki.gnome.org/Projects/Vala) can directly make use of this.

`test.vala` contains some example usage of the API and `make run-vala` runs this.

### Usage from other languages

The [gobject-introspection](https://gitlab.gnome.org/GNOME/gobject-introspection/)
`.gir` and `.typelib` files can be used to autogenerate bindings for dozens of
different languages.

