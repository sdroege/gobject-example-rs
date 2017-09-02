use glib_ffi;
use gobject_ffi;
use foo;
use libc::c_char;

#[repr(C)]
pub struct Bar {
    pub parent: foo::imp::Foo,
}

#[repr(C)]
pub struct BarClass {
    pub parent_class: foo::imp::FooClass,
}

extern "C" {
    pub fn ex_bar_new(name: *const c_char) -> *mut Bar;
    pub fn ex_bar_get_type() -> glib_ffi::GType;

    pub fn ex_bar_set_number(this: *mut Bar, num: f64);
    pub fn ex_bar_get_number(this: *mut Bar) -> f64;
}
