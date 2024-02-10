use glib::{prelude::*, translate::*};

#[derive(Debug, Copy, Clone, glib::Enum)]
#[enum_type(name = "ExColor")]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub(crate) mod ffi {
    use super::*;

    pub type ExColor = <super::Color as super::IntoGlib>::GlibType;

    pub const EX_COLOR_RED: ExColor = super::Color::Red as i32;
    pub const EX_COLOR_GREEN: ExColor = super::Color::Green as i32;
    pub const EX_COLOR_BLUE: ExColor = super::Color::Blue as i32;

    #[no_mangle]
    pub unsafe extern "C" fn ex_color_get_type() -> glib::ffi::GType {
        super::Color::static_type().into_glib()
    }
}
