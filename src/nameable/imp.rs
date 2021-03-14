use std::ptr;

use glib;
use glib::subclass::prelude::*;
use glib::translate::{from_glib_borrow, FromGlibPtrBorrow, ToGlib, ToGlibPtr};

use super::Nameable as NameableWrapper;

use libc::{c_char, c_void};

// Instance struct
pub struct Nameable(c_void);

// Interface struct aka "vtable"
//
// Here we would store virtual methods and similar
#[derive(Clone, Copy)]
#[repr(C)]
pub struct NameableInterface {
    pub parent_iface: glib::gobject_ffi::GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut Nameable) -> *mut c_char>,
}

#[glib::object_interface]
unsafe impl ObjectInterface for NameableInterface {
    const NAME: &'static str = "ExNameableInterface";
    type Prerequisites = (glib::Object,);

    // Interface struct initialization, called from GObject
    fn interface_init(&mut self) {
        // TODO: Could also add signals here, and interface properties via
        // g_object_interface_install_property()
        self.get_name = Some(get_name_default_trampoline);
    }
}

//
// Virtual method implementations / trampolines to safe implementations
//
// The default implementations are optional!
//
unsafe extern "C" fn get_name_default_trampoline(this: *mut Nameable) -> *mut c_char {
    NameableInterface::get_name_default(&from_glib_borrow(this)).to_glib_full()
}

//
// Safe implementations. These take the wrapper type, and not &Self, as first argument
//
impl NameableInterface {
    fn get_name_default(_this: &NameableWrapper) -> Option<String> {
        None
    }
}

//
// Public C functions below
//
#[no_mangle]
pub unsafe extern "C" fn ex_nameable_get_type() -> glib::ffi::GType {
    NameableInterface::get_type().to_glib()
}

// Virtual method callers
#[no_mangle]
pub unsafe extern "C" fn ex_nameable_get_name(this: *mut Nameable) -> *mut c_char {
    let wrapper = NameableWrapper::from_glib_borrow(this);
    let iface = NameableInterface::from_instance(&*wrapper);
    iface.get_name.map(|f| f(this)).unwrap_or(ptr::null_mut())
}
