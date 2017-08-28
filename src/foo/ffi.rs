use glib_ffi;
use gobject_ffi;

#[repr(C)]
pub struct Foo {
    parent: gobject_ffi::GObject,
}

#[repr(C)]
pub struct FooClass {
    parent_class: gobject_ffi::GObjectClass,
}

extern "C" {
    pub fn ex_foo_new() -> *mut Foo;
    pub fn ex_foo_get_type() -> glib_ffi::GType;
}
