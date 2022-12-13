#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
pub(crate) use imp::ffi;

#[cfg(feature = "bindings")]
pub(crate) mod ffi;

use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::subclass::prelude::*;
use glib::translate::*;

use std::mem;
use std::pin::Pin;

use crate::nameable::Nameable;

#[cfg(feature = "bindings")]
glib::wrapper! {
    pub struct Foo(Object<ffi::ExFoo, ffi::ExFooClass>) @implements Nameable;

    match fn {
        type_ => || ffi::ex_foo_get_type(),
    }
}

#[cfg(not(feature = "bindings"))]
glib::wrapper! {
    pub struct Foo(ObjectSubclass<imp::Foo>) @implements Nameable;
}

impl Foo {
    pub fn new(name: Option<&str>) -> Foo {
        unsafe { from_glib_full(ffi::ex_foo_new(name.to_glib_none().0)) }
    }
}

pub trait FooExt {
    fn increment(&self, inc: i32) -> i32;
    fn counter(&self) -> i32;
    fn name(&self) -> Option<String>;

    fn property_name(&self) -> Option<String>;

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn check_future(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;
}

impl<O: IsA<Foo>> FooExt for O {
    fn increment(&self, inc: i32) -> i32 {
        unsafe { ffi::ex_foo_increment(self.as_ref().to_glib_none().0, inc) }
    }

    fn counter(&self) -> i32 {
        unsafe { ffi::ex_foo_get_counter(self.as_ref().to_glib_none().0) }
    }

    fn name(&self) -> Option<String> {
        unsafe { from_glib_full(ffi::ex_foo_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn property_name(&self) -> Option<String> {
        let mut value = glib::Value::for_value_type::<String>();
        unsafe {
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
        }
        value.get().unwrap()
    }

    fn connect_incremented<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn connect_incremented_trampoline<P, F: Fn(&P, i32, i32) + 'static>(
            this: *mut ffi::ExFoo,
            val: i32,
            inc: i32,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Foo>,
        {
            let f = &*(f as *const F);
            f(
                &*Foo::from_glib_borrow(this).unsafe_cast_ref::<P>(),
                val,
                inc,
            )
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"incremented\0".as_ptr() as *const _,
                Some(mem::transmute(
                    connect_incremented_trampoline::<Self, F> as usize,
                )),
                Box::into_raw(f),
            )
        }
    }

    fn check_future(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        fn check_async<O: IsA<Foo>, P: FnOnce(Result<(), glib::Error>) + 'static>(
            obj: &O,
            cancellable: Option<&impl IsA<gio::Cancellable>>,
            callback: P,
        ) {
            let main_context = glib::MainContext::ref_thread_default();
            let is_main_context_owner = main_context.is_owner();
            let has_acquired_main_context = (!is_main_context_owner)
                .then(|| main_context.acquire().ok())
                .flatten();
            assert!(
                is_main_context_owner || has_acquired_main_context.is_some(),
                "Async operations only allowed if the thread is owning the MainContext"
            );

            let user_data: Box<glib::thread_guard::ThreadGuard<P>> =
                Box::new(glib::thread_guard::ThreadGuard::new(callback));
            unsafe extern "C" fn check_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
                _source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) {
                let mut error = std::ptr::null_mut();
                let _ = ffi::ex_foo_check_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };
                let callback: Box<glib::thread_guard::ThreadGuard<P>> =
                    Box::from_raw(user_data as *mut _);
                let callback: P = callback.into_inner();
                callback(result);
            }
            let callback = check_trampoline::<P>;
            unsafe {
                ffi::ex_foo_check_async(
                    obj.as_ref().to_glib_none().0,
                    cancellable.map(|p| p.as_ref()).to_glib_none().0,
                    Some(callback),
                    Box::into_raw(user_data) as *mut _,
                );
            }
        }

        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            check_async(obj, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
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
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ExFooClass;
            if let Some(ref f) = (*parent_class).increment {
                f(obj.to_glib_none().0, inc)
            } else {
                unimplemented!()
            }
        }
    }

    fn parent_incremented(&self, obj: &Foo, val: i32, inc: i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ExFooClass;
            if let Some(ref f) = (*parent_class).incremented {
                f(obj.to_glib_none().0, val, inc)
            }
        }
    }
}

unsafe impl<T: FooImpl> IsSubclassable<T> for Foo {
    fn class_init(class: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.increment = Some(increment_trampoline::<T>);
        klass.incremented = Some(incremented_trampoline::<T>);
    }
    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

// Virtual method default implementation trampolines
unsafe extern "C" fn increment_trampoline<T: ObjectSubclass>(this: *mut ffi::ExFoo, inc: i32) -> i32
where
    T: FooImpl,
{
    let instance = &*(this as *const T::Instance);
    let imp = instance.imp();
    imp.increment(&from_glib_borrow(this), inc)
}

unsafe extern "C" fn incremented_trampoline<T: ObjectSubclass>(
    this: *mut ffi::ExFoo,
    val: i32,
    inc: i32,
) where
    T: FooImpl,
{
    let instance = &*(this as *const T::Instance);
    let imp = instance.imp();
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

        assert_eq!(foo.counter(), 0);
        assert_eq!(foo.increment(1), 1);
        assert_eq!(*incremented.borrow(), (1, 1));
        assert_eq!(foo.counter(), 1);
        assert_eq!(foo.increment(10), 11);
        assert_eq!(*incremented.borrow(), (11, 10));
        assert_eq!(foo.counter(), 11);
    }

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        assert_eq!(foo.name(), Some("foo's name".into()));
        assert_eq!(foo.property_name(), Some("foo's name".into()));
    }

    #[test]
    fn test_async() {
        let foo = Foo::new(Some("foo's name"));

        let c = glib::MainContext::default();
        let l = glib::MainLoop::new(Some(&c), false);

        let future = glib::clone!(@strong l => async move {
            assert!(foo.check_future().await.is_ok());
            l.quit();
        });

        c.spawn_local(future);
        l.run();
    }
}
