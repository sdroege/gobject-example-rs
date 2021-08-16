#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

#[cfg(feature = "bindings")]
mod ffi;

use glib::{StaticType, Type, translate::*};

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const SOME = ffi::EX_FLAGS_SOME;
        const ZING = ffi::EX_FLAGS_ZING;
        const BONG = ffi::EX_FLAGS_BONG;
    }
}

impl IntoGlib for Flags {
    type GlibType = ffi::ExFlags;

    fn into_glib(self) -> ffi::ExFlags {
        self.bits()
    }
}

impl FromGlib<ffi::ExFlags> for Flags {
    unsafe fn from_glib(value: ffi::ExFlags) -> Self {
        Flags::from_bits_truncate(value)
    }
}

impl StaticType for Flags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ex_flags_get_type()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags() {
        assert_eq!(Flags::ZING.bits(), 2);
        let t = Flags::static_type();
        assert!(t.is_a(glib::Type::FLAGS));
        assert_eq!(t.name(), "ExFlags");
        let e = glib::FlagsClass::new(t).unwrap();
        let v = e.value(1).unwrap();
        assert_eq!(v.name(), "Some");
        assert_eq!(v.nick(), "some");

    }
}
