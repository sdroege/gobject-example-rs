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

use glib;
use glib::Value;
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
    pub fn new(name: Option<&str>) -> Foo {
        unsafe { from_glib_full(imp::ex_foo_new(name.to_glib_none().0)) }
    }
}

pub trait FooExt {
    fn increment(&self, inc: i32) -> i32;
    fn get_counter(&self) -> i32;
    fn get_name(&self) -> Option<String>;

    fn get_property_name(&self) -> Option<String>;
}

impl<O: IsA<Foo> + IsA<glib::object::Object>> FooExt for O {
    fn increment(&self, inc: i32) -> i32 {
        unsafe { imp::ex_foo_increment(self.to_glib_none().0, inc) }
    }

    fn get_counter(&self) -> i32 {
        unsafe { imp::ex_foo_get_counter(self.to_glib_none().0) }
    }

    fn get_name(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_foo_get_name(self.to_glib_none().0)) }
    }

    fn get_property_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "name".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let foo = Foo::new(Some("foo's name"));

        drop(foo);
    }

    #[test]
    fn test_counter() {
        let foo = Foo::new(Some("foo's name"));

        assert_eq!(foo.get_counter(), 0);
        assert_eq!(foo.increment(1), 1);
        assert_eq!(foo.get_counter(), 1);
        assert_eq!(foo.increment(10), 11);
        assert_eq!(foo.get_counter(), 11);
    }

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        assert_eq!(foo.get_name(), Some("foo's name".into()));
        assert_eq!(foo.get_property_name(), Some("foo's name".into()));
    }
}
