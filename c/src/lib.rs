use imsz::{ImError};
use std::os::raw::{c_int, c_char};

#[repr(C)]
pub struct ImInfoC {
    format: c_int,
    width:  u64,
    height: u64,
}

#[no_mangle]
pub extern "C" fn imsz(fname: *const c_char, info_ptr: *mut ImInfoC) -> c_int {
    let fname = unsafe { std::ffi::CStr::from_ptr(fname) };
    let fname = Vec::from(fname.to_bytes());
    let fname = unsafe { String::from_utf8_unchecked(fname) };
    match imsz::imsz(fname) {
        Ok(info) => {
            if info_ptr != std::ptr::null_mut() {
                unsafe {
                    (*info_ptr).format = info.format as c_int;
                    (*info_ptr).width  = info.width;
                    (*info_ptr).height = info.height;
                }
            }
            return 0;
        },
        Err(ImError::IO(error)) => {
            if let Some(errnum) = error.raw_os_error() {
                return errnum;
            } else {
                return -1;
            }
        },
        Err(ImError::ParserError(format)) => {
            if info_ptr != std::ptr::null_mut() {
                unsafe {
                    (*info_ptr).format = format as c_int;
                }
            }
            return -2;
        },
        Err(ImError::UnknownFormat) => {
            return -3;
        }
    }
}
