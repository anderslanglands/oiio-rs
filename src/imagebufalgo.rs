use crate::ffi;
pub use crate::ffi::CompareResults;
use crate::imagebuf::ImageBuf;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

pub fn compare(
    a: &ImageBuf,
    b: &ImageBuf,
    failthresh: f32,
    warnthresh: f32,
) -> CompareResults {
    unsafe { ffi::ImageBufAlgo_compare(a.buf, b.buf, failthresh, warnthresh) }
}

pub fn colorconvert(
    src: &ImageBuf,
    fromspace: &str,
    tospace: &str,
) -> ImageBuf {
    let fromspace = CString::new(fromspace).unwrap();
    let tospace = CString::new(tospace).unwrap();
    unsafe {
        ImageBuf {
            buf: ffi::ImageBufAlgo_colorconvert(
                src.buf,
                fromspace.as_ptr(),
                tospace.as_ptr(),
            ),
            data: None,
        }
    }
}
