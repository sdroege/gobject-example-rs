#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

#[cfg(feature = "bindings")]
mod ffi;

use glib::{prelude::*, subclass::prelude::*, translate::*};

#[cfg(feature = "bindings")]
glib::wrapper! {
    pub struct Nameable(Interface<ffi::Nameable, ffi::Interface>);

    match fn {
        type_ => || ffi::ex_nameable_get_type(),
    }
}

#[cfg(not(feature = "bindings"))]
glib::wrapper! {
    pub struct Nameable(ObjectInterface<imp::Nameable>);
}

pub trait NameableExt: IsA<Nameable> {
    fn name(&self) -> Option<String> {
        let iface = self.interface::<Nameable>().unwrap();
        unsafe {
            from_glib_full(((iface.as_ref()).get_name.unwrap())(
                self.upcast_ref::<Nameable>().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<Nameable>> NameableExt for O {}

pub trait NameableImpl: ObjectImpl {
    fn name(&self) -> Option<String> {
        self.parent_name()
    }
}

pub trait NameableImplExt: NameableImpl {
    fn parent_name(&self) -> Option<String> {
        let data = Self::type_data();
        let parent_iface = unsafe {
            &*(data.as_ref().parent_interface::<Nameable>() as *const ffi::ExNameableInterface)
        };

        unsafe {
            from_glib_full((parent_iface.get_name.unwrap())(
                self.obj().unsafe_cast_ref::<Nameable>().to_glib_none().0,
            ))
        }
    }
}

impl<T: NameableImpl> NameableImplExt for T {}

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
    use super::*;
    use crate::foo::Foo;
    use crate::nameable::{Nameable, NameableExt};

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        // We cast here because otherwise we would just use the get_name() of foo itself
        let nameable = foo.upcast::<Nameable>();

        assert_eq!(nameable.name(), Some("foo's name".into()));
    }
}
