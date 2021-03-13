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
use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::subclass::prelude::*;
use glib::translate::*;

use std::mem;

use nameable;

glib::wrapper! {
    pub struct Foo(Object<imp::Foo, imp::FooClass, FooClass>) @implements nameable::Nameable;

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

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Foo>> FooExt for O {
    fn increment(&self, inc: i32) -> i32 {
        unsafe { imp::ex_foo_increment(self.as_ref().to_glib_none().0, inc) }
    }

    fn get_counter(&self) -> i32 {
        unsafe { imp::ex_foo_get_counter(self.as_ref().to_glib_none().0) }
    }

    fn get_name(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_foo_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn get_property_name(&self) -> Option<String> {
        let mut value = glib::Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut gobject_ffi::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
        }
        value.get()
    }

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut gobject_ffi::GObject,
                b"incremented\0".as_ptr() as *const _,
                Some(mem::transmute(
                    connect_incremented_trampoline::<Self, F> as usize,
                )),
                Box::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn connect_incremented_trampoline<P, F: Fn(&P, i32, i32) + 'static>(
    this: *mut imp::Foo,
    val: i32,
    inc: i32,
    f: glib_ffi::gpointer,
) where
    P: IsA<Foo>,
{
    let f: &F = &*(f as *const F);
    f(&Foo::from_glib_borrow(this).unsafe_cast(), val, inc)
}

pub trait FooImpl: ObjectImpl + 'static {
    fn increment(&self, obj: &Foo, inc: i32) -> i32 {
        self.parent_increment(obj, inc)
    }

    fn incremented(&self, obj: &Foo, val: i32, inc: i32) {
        self.parent_incremented(obj, val, inc);
    }

    fn parent_increment(&self, obj: &Foo, inc: i32) -> i32 {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut imp::FooClass;
            if let Some(ref f) = (*parent_class).increment {
                f(obj.to_glib_none().0, inc)
            } else {
                unimplemented!()
            }
        }
    }

    fn parent_incremented(&self, obj: &Foo, val: i32, inc: i32) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut imp::FooClass;
            if let Some(ref f) = (*parent_class).incremented {
                f(obj.to_glib_none().0, val, inc)
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + FooImpl> IsSubclassable<T> for FooClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut imp::FooClass);
            klass.increment = Some(increment_trampoline::<T>);
            klass.incremented = Some(incremented_trampoline::<T>);
        }
    }
}

// Virtual method default implementation trampolines
unsafe extern "C" fn increment_trampoline<T: ObjectSubclass>(this: *mut imp::Foo, inc: i32) -> i32
where
    T: FooImpl,
{
    let instance = &*(this as *const T::Instance);
    let imp = instance.get_impl();
    imp.increment(&from_glib_borrow(this), inc)
}

unsafe extern "C" fn incremented_trampoline<T: ObjectSubclass>(
    this: *mut imp::Foo,
    val: i32,
    inc: i32,
) where
    T: FooImpl,
{
    let instance = &*(this as *const T::Instance);
    let imp = instance.get_impl();
    imp.incremented(&from_glib_borrow(this), val, inc);
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
