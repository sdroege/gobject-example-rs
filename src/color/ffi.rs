use libc::c_int;

pub type ExColor = c_int;

pub const EX_COLOR_RED: ExColor = 0;
pub const EX_COLOR_GREEN: ExColor = 1;
pub const EX_COLOR_BLUE: ExColor = 2;

extern "C" {
    pub fn ex_color_get_type() -> glib::ffi::GType;
}
