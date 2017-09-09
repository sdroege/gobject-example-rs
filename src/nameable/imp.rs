use glib_ffi;
use gobject_ffi;

use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use std::ptr;

use glib::translate::{from_glib_borrow, ToGlibPtr};

use libc::{c_char, c_void};

use nameable::Nameable as NameableWrapper;

// Instance struct
#[repr(C)]
pub struct Nameable(c_void);

// Interface struct aka "vtable"
//
// Here we would store virtual methods and similar
#[repr(C)]
pub struct NameableInterface {
    pub parent_iface: gobject_ffi::GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut Nameable) -> *mut c_char>,
}

impl Nameable {
    // Helper functions
    fn get_iface(&self) -> &NameableInterface {
        unsafe {
            let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
            let interface =
                gobject_ffi::g_type_interface_peek(klass as *mut c_void, ex_nameable_get_type());
            &*(interface as *const NameableInterface)
        }
    }

    //
    // Virtual method implementations / trampolines to safe implementations
    //
    // The default implementations are optional!
    //
    // FIXME: There is no private data here for the default implementation! If
    // we needed one, it could be done via qdata on the instance
    //
    unsafe extern "C" fn get_name_default_trampoline(this: *mut Nameable) -> *mut c_char {
        callback_guard!();

        Nameable::get_name_default(&from_glib_borrow(this)).to_glib_full()
    }

    //
    // Safe implementations. These take the wrapper type, and not &Self, as first argument
    //
    fn get_name_default(_this: &NameableWrapper) -> Option<String> {
        None
    }
}

impl NameableInterface {
    // Interface struct initialization, called from GObject
    unsafe extern "C" fn init(interface: glib_ffi::gpointer, _interface_data: glib_ffi::gpointer) {
        callback_guard!();

        // TODO: Could also add signals here, and interface properties via
        // g_object_interface_install_property()
        {
            let nameable_iface = &mut *(interface as *mut NameableInterface);
            nameable_iface.get_name = Some(Nameable::get_name_default_trampoline);
        }
    }
}

//
// Public C functions below
//

// Virtual method callers
#[no_mangle]
pub unsafe extern "C" fn ex_nameable_get_name(this: *mut Nameable) -> *mut c_char {
    callback_guard!();

    let iface = (*this).get_iface();
    iface.get_name.map(|f| f(this)).unwrap_or(ptr::null_mut())
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_nameable_get_type() -> glib_ffi::GType {
    callback_guard!();

    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_info = gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<NameableInterface>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(NameableInterface::init),
            class_finalize: None,
            class_data: ptr::null(),
            instance_size: 0,
            n_preallocs: 0,
            instance_init: None,
            value_table: ptr::null(),
        };

        let type_name = CString::new("ExNameable").unwrap();

        TYPE = gobject_ffi::g_type_register_static(
            gobject_ffi::G_TYPE_INTERFACE,
            type_name.as_ptr(),
            &type_info,
            gobject_ffi::GTypeFlags::empty(),
        );

        // Can add pre-requisites on base classes of implementors and other interfaces
        // they must implement here
        gobject_ffi::g_type_interface_add_prerequisite(TYPE, gobject_ffi::g_object_get_type());
    });

    TYPE
}
