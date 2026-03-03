//! FFI bindings for AgenticCognition
//!
//! Provides C-compatible interface for Python and WASM bindings.

use agentic_cognition::CognitionStore;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::PathBuf;

/// Create a new cognition store
///
/// # Safety
/// `path` must be a valid null-terminated C string
#[no_mangle]
pub unsafe extern "C" fn acog_store_new(path: *const c_char) -> *mut CognitionStore {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match CognitionStore::with_storage(PathBuf::from(path_str)) {
        Ok(store) => Box::into_raw(Box::new(store)),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a cognition store
///
/// # Safety
/// `store` must have been created by `acog_store_new`
#[no_mangle]
pub unsafe extern "C" fn acog_store_free(store: *mut CognitionStore) {
    if !store.is_null() {
        unsafe { drop(Box::from_raw(store)) };
    }
}

/// Get the version string
#[no_mangle]
pub extern "C" fn acog_version() -> *const c_char {
    static VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const c_char
}
