use libc::c_uint;

pub type ExFlags = c_uint;

pub const EX_FLAGS_SOME: ExFlags = 1;
pub const EX_FLAGS_ZING: ExFlags = 2;
pub const EX_FLAGS_BONG: ExFlags = 4;

extern "C" {
    pub fn ex_flags_get_type() -> glib::ffi::GType;
}
