extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

#[macro_use]
extern crate glib;

extern crate libc;

pub mod bar;
pub mod foo;
pub mod nameable;
pub mod rstring;
pub mod shared_rstring;
