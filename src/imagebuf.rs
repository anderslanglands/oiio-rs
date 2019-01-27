use crate::ffi;
use crate::typedesc::TypeDesc;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

pub struct ImageBuf {
    pub(crate) buf: ffi::ImageBuf,
}

impl Drop for ImageBuf {
    fn drop(&mut self) {
        unsafe { ffi::ImageBuf_destroy(self.buf) };
    }
}

impl ImageBuf {
    pub fn create(filename: &str) -> Result<ImageBuf, String> {
        let filename = CString::new(filename).unwrap();
        let buf = unsafe { ffi::ImageBuf_create(filename.as_ptr()) };

        if buf.is_null() {
            Err(format!("Could not create ImageBuf for {:?}", filename))
        } else {
            Ok(ImageBuf { buf })
        }
    }

    pub fn read(&self) -> Result<(), String> {
        let result = unsafe { ffi::ImageBuf_read(self.buf) };
        if result {
            Ok(())
        } else {
            Err(format!("Could not read image buf"))
        }
    }

    pub fn write(&self, filename: &str, dtype: TypeDesc) -> Result<(), String> {
        let cfilename = CString::new(filename).unwrap();
        let result = unsafe { ffi::ImageBuf_write(self.buf, cfilename.as_ptr(), dtype) };
        if result {
            Ok(())
        } else {
            Err(format!("Could not write image buf to \"{}\"", filename))
        }
    }
}
