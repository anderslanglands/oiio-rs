use crate::ffi;
use crate::imageio;
use crate::typedesc::TypeDesc;
use std::ffi::CString;
use std::os::raw::c_void;

pub enum DataVec {
    F32(Vec<f32>),
}

pub trait WrappedVec {
    fn wrap(self) -> DataVec;
    fn as_void_ptr(&self) -> *const c_void;
}

impl WrappedVec for Vec<f32> {
    fn wrap(self) -> DataVec {
        DataVec::F32(self)
    }
    fn as_void_ptr(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

pub struct ImageBuf {
    pub(crate) buf: ffi::ImageBuf,
    pub(crate) _data: Option<DataVec>,
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
            Ok(ImageBuf { buf, _data: None })
        }
    }

    pub fn create_with_data<V: WrappedVec>(
        spec: imageio::ImageSpec,
        data: V,
    ) -> Result<ImageBuf, String> {
        let buf = unsafe {
            ffi::ImageBuf_create_with_data(spec.spec, data.as_void_ptr())
        };

        Ok(ImageBuf {
            buf,
            _data: Some(data.wrap()),
        })
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
        let result =
            unsafe { ffi::ImageBuf_write(self.buf, cfilename.as_ptr(), dtype) };
        if result {
            Ok(())
        } else {
            Err(format!("Could not write image buf to \"{}\"", filename))
        }
    }
}
