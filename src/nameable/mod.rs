#[cfg(not(feature = "bindings"))]
pub mod imp;

#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use nameable::ffi::*;
}

use glib_ffi;
use gobject_ffi;

use glib;
use glib::IsA;
use glib::translate::*;

use std::ptr;
use std::mem;

glib_wrapper! {
    pub struct Nameable(Object<imp::Nameable>);

    match fn {
        get_type => || imp::ex_nameable_get_type(),
    }
}

pub trait NameableExt {
    fn get_name(&self) -> Option<String>;
}

impl<O: IsA<Nameable> + IsA<glib::object::Object>> NameableExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_nameable_get_name(self.to_glib_none().0)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib::Cast;
    use foo::Foo;

    #[test]
    fn test_name() {
        let foo = Foo::new(Some("foo's name"));

        // We cast here because otherwise we would just use the get_name() of foo itself
        let nameable = foo.upcast::<Nameable>();

        assert_eq!(nameable.get_name(), Some("foo's name".into()));
    }
}
