#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use self::imp::ffi;

/// cbindgen:ignore
#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use super::ffi::*;
}

use glib::object::ObjectType;
use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;

use std::mem;

use crate::foo::Foo;
use crate::nameable::Nameable;

#[cfg(feature = "bindings")]
glib::wrapper! {
    pub struct Bar(Object<imp::Bar, imp::BarClass>) @extends Foo, @implements Nameable;

    match fn {
        get_type => || imp::ex_bar_get_type(),
    }
}

#[cfg(not(feature = "bindings"))]
glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>) @extends Foo, @implements Nameable;
}

impl Bar {
    pub fn new(name: Option<&str>) -> Bar {
        unsafe { from_glib_full(imp::ex_bar_new(name.to_glib_none().0)) }
    }

    pub fn set_number(&self, num: f64) {
        unsafe { imp::ex_bar_set_number(self.to_glib_none().0, num) }
    }

    pub fn get_number(&self) -> f64 {
        unsafe { imp::ex_bar_get_number(self.to_glib_none().0) }
    }

    pub fn get_property_number(&self) -> f64 {
        let mut value = glib::Value::from(&0.0f64);
        unsafe {
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"number\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
        }
        value.get().unwrap().unwrap()
    }

    pub fn set_property_number(&self, num: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"number\0".as_ptr() as *const _,
                glib::Value::from(&num).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_number_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::Bar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Bar>,
        {
            let f = &*(f as *const F);
            f(&*Bar::from_glib_borrow(this).unsafe_cast_ref::<P>())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"notify::number\0".as_ptr() as *const _,
                Some(mem::transmute(notify_number_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foo::FooExt;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        let bar = Bar::new(Some("bar's name"));

        drop(bar);
    }

    #[test]
    fn test_counter() {
        let bar = Bar::new(Some("bar's name"));

        assert_eq!(bar.get_counter(), 0);
        assert_eq!(bar.increment(1), 2);
        assert_eq!(bar.get_counter(), 2);
        assert_eq!(bar.increment(10), 22);
        assert_eq!(bar.get_counter(), 22);
    }

    #[test]
    fn test_name() {
        let bar = Bar::new(Some("bar's name"));

        assert_eq!(bar.get_name(), Some("bar's name".into()));
        assert_eq!(bar.get_property_name(), Some("bar's name".into()));
    }

    #[test]
    fn test_number() {
        let bar = Bar::new(Some("bar's name"));

        let counter = Rc::new(RefCell::new(0i32));
        let counter_clone = counter.clone();
        bar.connect_property_number_notify(move |_| {
            *counter_clone.borrow_mut() += 1;
        });

        assert_eq!(*counter.borrow(), 0);
        assert_eq!(bar.get_number(), 0.0);
        assert_eq!(bar.get_property_number(), 0.0);
        bar.set_number(10.0);
        assert_eq!(*counter.borrow(), 1);
        assert_eq!(bar.get_number(), 10.0);
        assert_eq!(bar.get_property_number(), 10.0);
        bar.set_property_number(20.0);
        assert_eq!(*counter.borrow(), 2);
        assert_eq!(bar.get_number(), 20.0);
        assert_eq!(bar.get_property_number(), 20.0);
    }
}
