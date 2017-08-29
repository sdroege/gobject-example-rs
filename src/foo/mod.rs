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
use glib::{IsA, Value};
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;

use std::ptr;
use std::mem;
use std::mem::transmute;

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

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> u64;
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

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box<Box<Fn(&Self, i32, i32) + 'static>> = Box::new(Box::new(f));
            connect(
                self.to_glib_none().0,
                "incremented",
                transmute(incremented_trampoline::<Self> as usize),
                Box::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn incremented_trampoline<P>(
    this: *mut imp::Foo,
    val: i32,
    inc: i32,
    f: glib_ffi::gpointer,
) where
    P: IsA<Foo>,
{
    let f: &&(Fn(&Foo, i32, i32) + 'static) = transmute(f);
    f(&Foo::from_glib_none(this).downcast_unchecked(), val, inc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        let foo = Foo::new(Some("foo's name"));

        drop(foo);
    }

    #[test]
    fn test_counter() {
        let foo = Foo::new(Some("foo's name"));

        let incremented = Rc::new(RefCell::new((0i32, 0i32)));
        let incremented_clone = incremented.clone();
        foo.connect_incremented(move |_, val, inc| {
            *incremented_clone.borrow_mut() = (val, inc);
        });

        assert_eq!(foo.get_counter(), 0);
        assert_eq!(foo.increment(1), 1);
        assert_eq!(*incremented.borrow(), (1, 1));
        assert_eq!(foo.get_counter(), 1);
        assert_eq!(foo.increment(10), 11);
        assert_eq!(*incremented.borrow(), (11, 10));
        assert_eq!(foo.get_counter(), 11);
    }

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        assert_eq!(foo.get_name(), Some("foo's name".into()));
        assert_eq!(foo.get_property_name(), Some("foo's name".into()));
    }
}
