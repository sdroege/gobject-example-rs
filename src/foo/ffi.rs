use glib::ffi;
use glib::gobject_ffi;
use libc::c_char;

#[repr(C)]
pub struct Foo {
    pub parent: glib::gobject_ffi::GObject,
}

#[repr(C)]
pub struct FooClass {
    pub parent_class: glib::gobject_ffi::GObjectClass,
    pub increment: Option<unsafe extern "C" fn(*mut Foo, inc: i32) -> i32>,
    pub incremented: Option<unsafe extern "C" fn(*mut Foo, val: i32, inc: i32)>,
}

extern "C" {
    pub fn ex_foo_new(name: *const c_char) -> *mut Foo;
    pub fn ex_foo_get_type() -> glib::ffi::GType;

    pub fn ex_foo_increment(this: *mut Foo, inc: i32) -> i32;
    pub fn ex_foo_get_counter(this: *mut Foo) -> i32;
    pub fn ex_foo_get_name(this: *mut Foo) -> *mut c_char;
}
