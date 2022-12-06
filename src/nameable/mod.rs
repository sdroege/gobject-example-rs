#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

#[cfg(feature = "bindings")]
mod ffi;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::IsA;

glib::wrapper! {
    pub struct Nameable(Interface<ffi::ExNameable, ffi::ExNameableInterface>);

    match fn {
        type_ => || ffi::ex_nameable_get_type(),
    }
}

pub trait NameableExt {
    fn name(&self) -> Option<String>;
}

impl<O: IsA<Nameable>> NameableExt for O {
    fn name(&self) -> Option<String> {
        unsafe { from_glib_full(ffi::ex_nameable_get_name(self.as_ref().to_glib_none().0)) }
    }
}

pub trait NameableImpl: ObjectImpl {
    fn name(&self) -> Option<String>;
}

unsafe impl<T: ObjectSubclass + NameableImpl> IsImplementable<T> for Nameable {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_name = Some(get_name_trampoline::<T>);
    }
    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn get_name_trampoline<T: ObjectSubclass>(
    nameable: *mut ffi::ExNameable,
) -> *mut std::ffi::c_char
where
    T: NameableImpl,
{
    let instance = &*(nameable as *mut T::Instance);
    let imp = instance.imp();

    imp.name().to_glib_full()
}

#[cfg(test)]
mod tests {
    use crate::foo::Foo;
    use crate::nameable::{Nameable, NameableExt};
    use glib::Cast;

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        // We cast here because otherwise we would just use the get_name() of foo itself
        let nameable = foo.upcast::<Nameable>();

        assert_eq!(nameable.name(), Some("foo's name".into()));
    }
}
