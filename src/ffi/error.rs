use std::cell::RefCell;
use std::ffi::{c_char, CString};
use std::ptr;

use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ErrorCode {
    Success = 0,
    Fail = 1,
}

thread_local! {
    pub static CURRENT_ERROR_C_JSON: RefCell<Option<CString>> = RefCell::new(None);
}

pub fn fail(err: Error) -> ErrorCode {
    set_current_error(&err);
    ErrorCode::Fail
}

pub fn set_current_error(err: &Error) {
    use serde_json::json;

    CURRENT_ERROR_C_JSON.with(|error| {
        let error_json = json!({
            "message": err.to_string(),
        })
        .to_string();
        error.replace(Some(CString::new(error_json).unwrap()));
    });
}

pub fn get_current_error_c_json() -> *const c_char {
    CURRENT_ERROR_C_JSON
        .with(|err| err.borrow_mut().take().map(CString::into_raw))
        .unwrap_or(ptr::null_mut())
}

/// Get details for last occurred error.
///
/// NOTE: Error is stored until the next one occurs.
///       Returning pointer has the same lifetime.
///
/// #Params
/// * `error_json_p` - Reference that will contain error details (if any error has occurred before)
///  in the format:
/// {
///     "message": str - human-readable error description
/// }
///
#[no_mangle]
pub extern "C" fn indy_bls_get_current_error(error_json_p: *mut *const c_char) {
    trace!(
        "indy_bls_get_current_error >>> error_json_p: {:?}",
        error_json_p
    );

    let error = get_current_error_c_json();
    unsafe { *error_json_p = error };

    trace!("indy_bls_get_current_error: <<<");
}
