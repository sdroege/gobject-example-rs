#[cfg(not(feature = "bindings"))]
pub mod imp;

#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use foo::ffi::*;
}

use glib_ffi;
use gobject_ffi;

use glib::IsA;
use glib::translate::*;

use std::ptr;
use std::mem;

glib_wrapper! {
    pub struct Foo(Object<imp::Foo>);

    match fn {
        get_type => || imp::ex_foo_get_type(),
    }
}

impl Foo {
    pub fn new() -> Foo {
        unsafe { from_glib_full(imp::ex_foo_new()) }
    }
}

pub trait FooExt {
    fn increment(&self, inc: i32) -> i32;
    fn get_counter(&self) -> i32;
    fn get_name(&self) -> Option<String>;
}

impl<O: IsA<Foo>> FooExt for O {
    fn increment(&self, inc: i32) -> i32 {
        unsafe { imp::ex_foo_increment(self.to_glib_none().0, inc) }
    }

    fn get_counter(&self) -> i32 {
        unsafe { imp::ex_foo_get_counter(self.to_glib_none().0) }
    }

    fn get_name(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_foo_get_name(self.to_glib_none().0)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let foo = Foo::new();

        drop(foo);
    }

    #[test]
    fn test_counter() {
        let foo = Foo::new();

        assert_eq!(foo.get_counter(), 0);
        assert_eq!(foo.increment(1), 1);
        assert_eq!(foo.get_counter(), 1);
        assert_eq!(foo.increment(10), 11);
        assert_eq!(foo.get_counter(), 11);
    }

    #[test]
    fn test_name() {
        let foo = Foo::new();

        assert_eq!(foo.get_name(), None);
    }
}
