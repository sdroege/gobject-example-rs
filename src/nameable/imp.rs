use glib::subclass::prelude::*;
use glib::translate::*;

use std::ffi::c_char;

// Type implementing ObjectInterface. Use a uninhabited enum to make the type uninstantiatable.
pub enum Nameable {}

// Default implementation of the interface methods (note: these are optional)
impl Nameable {
    fn name_default() -> Option<String> {
        None
    }
}

#[glib::object_interface]
impl ObjectInterface for Nameable {
    const NAME: &'static str = "ExNameable";
    type Instance = ffi::ExNameable;
    type Interface = ffi::ExNameableInterface;
    type Prerequisites = (glib::Object,);

    // Interface struct initialization, called from GObject
    fn interface_init(iface: &mut Self::Interface) {
        // Optionally set the default implementation
        iface.get_name = Some(get_name_default_trampoline);

        // TODO: Could also add signals here, and interface properties via
        // g_object_interface_install_property()
    }
}

// trampoline to safe implementation
unsafe extern "C" fn get_name_default_trampoline(_this: *mut ffi::ExNameable) -> *mut c_char {
    Nameable::name_default().to_glib_full()
}

pub(crate) mod ffi {
    use super::*;
    use glib::object::ObjectExt;
    use std::ffi::c_char;
    use std::ptr;

    // Instance struct, to be used as pointer to "self" in ffi methods
    #[repr(C)]
    pub struct ExNameable(std::ffi::c_void);

    // Interface struct aka "vtable"
    //
    // Here we would store virtual methods and similar
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct ExNameableInterface {
        pub parent_iface: glib::gobject_ffi::GTypeInterface,
        pub get_name: Option<unsafe extern "C" fn(*mut ExNameable) -> *mut c_char>,
    }

    unsafe impl InterfaceStruct for ExNameableInterface {
        type Type = super::Nameable;
    }

    #[no_mangle]
    pub extern "C" fn ex_nameable_get_type() -> glib::ffi::GType {
        <super::Nameable as ObjectInterfaceType>::type_().into_glib()
    }

    // Virtual method callers
    /// # Safety
    ///
    /// Must be a Nameable interface.
    #[no_mangle]
    pub unsafe extern "C" fn ex_nameable_get_name(this: *mut ExNameable) -> *mut c_char {
        let wrapper = from_glib_borrow::<_, super::super::Nameable>(this);
        let iface = wrapper.interface::<super::super::Nameable>().unwrap();
        iface
            .as_ref()
            .get_name
            .map(|f| f(this))
            .unwrap_or(ptr::null_mut())
    }
}
