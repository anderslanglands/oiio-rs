use crate::ffi;
pub use crate::ffi::CompareResults;
use crate::imagebuf::ImageBuf;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

pub fn compare(a: &ImageBuf, b: &ImageBuf, failthresh: f32, warnthresh: f32) -> CompareResults {
    unsafe { ffi::ImageBufAlgo_compare(a.buf, b.buf, failthresh, warnthresh) }
}
