use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

use log::{LevelFilter, Metadata, Record};
use once_cell::sync::OnceCell;

use super::error::{fail, ErrorCode};

static LOGGER: OnceCell<CustomLogger> = OnceCell::new();

pub type EnabledCallback = extern "C" fn(context: *const c_void, level: i32) -> i8;

pub type LogCallback = extern "C" fn(
    context: *const c_void,
    level: i32,
    target: *const c_char,
    message: *const c_char,
    module_path: *const c_char,
    file: *const c_char,
    line: i32,
);

pub type FlushCallback = extern "C" fn(context: *const c_void);

pub struct CustomLogger {
    context: *const c_void,
    enabled: Option<EnabledCallback>,
    log: LogCallback,
    flush: Option<FlushCallback>,
    disabled: AtomicBool,
}

impl CustomLogger {
    fn new(
        context: *const c_void,
        enabled: Option<EnabledCallback>,
        log: LogCallback,
        flush: Option<FlushCallback>,
    ) -> Self {
        CustomLogger {
            context,
            enabled,
            log,
            flush,
            disabled: AtomicBool::new(false),
        }
    }

    fn disable(&self) {
        self.disabled.store(true, Ordering::Release);
    }
}

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        if self.disabled.load(Ordering::Acquire) {
            false
        } else if let Some(enabled_cb) = self.enabled {
            enabled_cb(self.context, metadata.level() as i32) != 0
        } else {
            true
        }
    }

    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let log_cb = self.log;

        let level = record.level() as i32;
        let target = CString::new(record.target()).unwrap();
        let message = CString::new(record.args().to_string()).unwrap();

        let module_path = record.module_path().map(|s| CString::new(s).unwrap());
        let file = record.file().map(|s| CString::new(s).unwrap());
        let line = record.line().unwrap_or(0) as i32;

        log_cb(
            self.context,
            level,
            target.as_ptr(),
            message.as_ptr(),
            module_path
                .as_ref()
                .map(|s| s.as_ptr())
                .unwrap_or(ptr::null_mut()),
            file.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null_mut()),
            line,
        )
    }

    fn flush(&self) {
        if let Some(flush) = self.flush {
            flush(self.context)
        }
    }
}

unsafe impl Send for CustomLogger {}
unsafe impl Sync for CustomLogger {}

#[no_mangle]
pub extern "C" fn indy_bls_set_custom_logger(
    context: *const c_void,
    log: LogCallback,
    enabled: Option<EnabledCallback>,
    flush: Option<FlushCallback>,
    max_level: i32,
) -> ErrorCode {
    let max_level = if let Some(level) = get_level_filter(max_level) {
        level
    } else {
        return fail(err_msg!("Invalid log level"));
    };
    if LOGGER
        .set(CustomLogger::new(context, enabled, log, flush))
        .is_err()
    {
        return fail(err_msg!("Repeated logger initialization"));
    }
    if log::set_logger(LOGGER.get().unwrap()).is_err() {
        fail(err_msg!("Repeated logger initialization"));
    }
    log::set_max_level(max_level);
    debug!("Initialized custom logger");
    ErrorCode::Success
}

#[no_mangle]
pub extern "C" fn indy_bls_clear_custom_logger() {
    debug!("Removing custom logger");
    if let Some(logger) = LOGGER.get() {
        logger.disable();
    }
}

#[no_mangle]
pub extern "C" fn indy_bls_set_default_logger() -> ErrorCode {
    if let Err(_) = env_logger::try_init() {
        return fail(err_msg!("Repeated logger initialization"));
    }
    debug!("Initialized default logger");
    ErrorCode::Success
}

#[no_mangle]
pub extern "C" fn askar_set_max_log_level(max_level: i32) -> ErrorCode {
    if let Some(level) = get_level_filter(max_level) {
        log::set_max_level(level);
        ErrorCode::Success
    } else {
        fail(err_msg!("Invalid log level"))
    }
}

fn get_level_filter(max_level: i32) -> Option<LevelFilter> {
    Some(match max_level {
        -1 => {
            // load from RUST_LOG environment variable
            // defaults to ERROR if unspecified
            env_logger::Logger::from_default_env().filter()
        }
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => return None,
    })
}
