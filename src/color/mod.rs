#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

#[cfg(feature = "bindings")]
mod ffi;

use glib::{StaticType, Type, translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum Color {
    Red,
    Green,
    Blue,
    __Unknown(i32),
}

impl IntoGlib for Color {
    type GlibType = ffi::ExColor;

    fn into_glib(self) -> ffi::ExColor {
        match self {
            Color::Red => ffi::EX_COLOR_RED,
            Color::Green => ffi::EX_COLOR_GREEN,
            Color::Blue => ffi::EX_COLOR_BLUE,
            Color::__Unknown(value) => value,
        }
    }
}

impl FromGlib<ffi::ExColor> for Color {
    unsafe fn from_glib(value: ffi::ExColor) -> Self {
        match value {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            value => Color::__Unknown(value),
        }
    }
}

impl StaticType for Color {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ex_color_get_type()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        let c = Color::Blue;
        assert_eq!(c.into_glib(), 2);

        let t = Color::static_type();
        assert!(t.is_a(glib::Type::ENUM));
        assert_eq!(t.name(), "ExColor");

        let e = glib::EnumClass::new(t).unwrap();
        let v = e.value(1).unwrap();
        assert_eq!(v.name(), "Green");
        assert_eq!(v.nick(), "green");

        assert_eq!(e.value(42), None);
    }
}
