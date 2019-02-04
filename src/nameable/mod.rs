#[cfg(not(feature = "bindings"))]
pub mod imp;

#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use nameable::ffi::*;
}

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::IsA;

glib_wrapper! {
    pub struct Nameable(Interface<imp::Nameable>);

    match fn {
        get_type => || imp::ex_nameable_get_type(),
    }
}

pub trait NameableExt {
    fn get_name(&self) -> Option<String>;
}

impl<O: IsA<Nameable>> NameableExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_nameable_get_name(self.as_ref().to_glib_none().0)) }
    }
}

pub trait NameableImpl: ObjectImpl + 'static {
    fn get_name(&self, nameable: &Nameable) -> Option<String>;
}

unsafe impl<T: ObjectSubclass + NameableImpl> IsImplementable<T> for Nameable {
    unsafe extern "C" fn interface_init(
        iface: glib_ffi::gpointer,
        _iface_data: glib_ffi::gpointer,
    ) {
        let iface = &mut *(iface as *mut imp::NameableInterface);
        iface.get_name = Some(get_name_trampoline::<T>);
    }
}

unsafe extern "C" fn get_name_trampoline<T: ObjectSubclass>(
    nameable: *mut imp::Nameable,
) -> *mut libc::c_char
where
    T: NameableImpl,
{
    let instance = &*(nameable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_name(&from_glib_borrow(nameable)).to_glib_full()
}

#[cfg(test)]
mod tests {
    use super::*;
    use foo::Foo;
    use glib::Cast;

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        // We cast here because otherwise we would just use the get_name() of foo itself
        let nameable = foo.upcast::<Nameable>();

        assert_eq!(nameable.get_name(), Some("foo's name".into()));
    }
}
