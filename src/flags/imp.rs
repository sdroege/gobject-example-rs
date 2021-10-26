use glib::gflags;

#[gflags("ExFlags")]
#[repr(C)]
pub enum Flags {
    SOME = 0b00000001,
    ZING = 0b00000010,
    BONG = 0b00000100,
}

pub(crate) mod ffi {
    use glib::translate::*;
    use glib::StaticType;

    pub type ExFlags = <super::Flags as IntoGlib>::GlibType;

    pub const EX_FLAGS_SOME: ExFlags = super::Flags::SOME.bits();
    pub const EX_FLAGS_ZING: ExFlags = super::Flags::ZING.bits();
    pub const EX_FLAGS_BONG: ExFlags = super::Flags::BONG.bits();

    #[no_mangle]
    pub unsafe extern "C" fn ex_flags_get_type() -> glib::ffi::GType {
        super::Flags::static_type().into_glib()
    }
}
