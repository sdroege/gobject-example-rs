use glib_ffi;
use gobject_ffi;
use foo;

#[repr(C)]
pub struct Bar {
    parent: foo::imp::Foo,
}

#[repr(C)]
pub struct BarClass {
    parent_class: foo::imp::FooClass,
}

extern "C" {
    pub fn ex_bar_new() -> *mut Bar;
    pub fn ex_bar_get_type() -> glib_ffi::GType;
}
