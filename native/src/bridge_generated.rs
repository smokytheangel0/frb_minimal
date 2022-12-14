#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_platform(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "platform",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(platform()),
    )
}

#[no_mangle]
pub extern "C" fn wire_rust_release_mode(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "rust_release_mode",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(rust_release_mode()),
    )
}

#[no_mangle]
pub extern "C" fn wire_search_local_items(port_: i64, input: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "search_local_items",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input = input.wire2api();
            move |task_callback| Ok(search_local_items(api_input))
        },
    )
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: wrapper structs

// Section: static checks

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: impl IntoDart

impl support::IntoDart for Item {
    fn into_dart(self) -> support::DartCObject {
        match self {
            Self::LargeChickenEgg => 0,
            Self::LargeChickenEggYolk => 1,
            Self::LargeChickenEggWhite => 2,
            Self::TableSalt => 3,
            Self::TableSugar => 4,
            Self::Water => 5,
            Self::WheatFlour => 6,
            Self::ActiveDryYeast => 7,
            Self::CowButter => 8,
        }
        .into_dart()
    }
}

impl support::IntoDart for Platform {
    fn into_dart(self) -> support::DartCObject {
        match self {
            Self::Unknown => 0,
            Self::Android => 1,
            Self::Ios => 2,
            Self::Windows => 3,
            Self::Unix => 4,
            Self::MacIntel => 5,
            Self::MacApple => 6,
            Self::Wasm => 7,
        }
        .into_dart()
    }
}
// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
