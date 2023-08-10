#[cfg(debug_assertions)]
macro_rules! secret {
    ($val:expr) => {{
        $val
    }};
}

#[cfg(not(debug_assertions))]
macro_rules! secret {
    ($val:expr) => {{
        "_"
    }};
}

macro_rules! check_useful_c_ptr {
    ($ptr:ident, $arg_pos:expr) => {
        if $ptr.is_null() {
            return fail(err_msg!(
                "Invalid pointer has been passed: argument {}",
                $arg_pos
            ));
        }
    };
}

macro_rules! check_useful_c_byte_array {
    ($ptr:ident, $len:expr, $arg1:expr, $arg2:expr) => {
        if $ptr.is_null() {
            return fail(err_msg!(
                "Invalid pointer has been passed: argument {}",
                $arg1
            ));
        }

        if $len <= 0 {
            return fail(err_msg!(
                "Array length must be greater than 0: argument {}",
                $arg2
            ));
        }

        let $ptr = unsafe { slice::from_raw_parts($ptr, $len) };
    };
}

macro_rules! check_useful_opt_c_byte_array {
    ($ptr:ident, $len:expr, $err1:expr, $err2:expr) => {
        if !$ptr.is_null() && $len <= 0 {
            return fail(err_msg!("Invalid pointer has been passed"));
        }

        let $ptr = if $ptr.is_null() {
            None
        } else {
            unsafe { Some(slice::from_raw_parts($ptr, $len)) }
        };
    };
}

macro_rules! check_useful_c_reference {
    ($ptr:ident, $type:ty, $arg_pos:expr) => {
        if $ptr.is_null() {
            return fail(err_msg!(
                "Invalid pointer has been passed: argument {}",
                $arg_pos
            ));
        }

        let $ptr: &$type = unsafe { &*($ptr as *const $type) };
    };
}

macro_rules! check_useful_c_reference_array {
    ($ptrs:ident, $ptrs_len:ident, $type:ty, $arg1:expr, $arg2:expr) => {
        if $ptrs.is_null() {
            return fail(err_msg!(
                "Invalid pointer has been passed: argument {}",
                $arg1
            ));
        }

        if $ptrs_len <= 0 {
            return fail(err_msg!(
                "Array length must be greater than 0: argument {}",
                $arg2
            ));
        }

        let $ptrs: Vec<&$type> = unsafe { slice::from_raw_parts($ptrs, $ptrs_len) }
            .iter()
            .map(|ptr| unsafe { &*(*ptr as *const $type) })
            .collect();
    };
}
