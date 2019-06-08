use crate::ffi;
use crate::imageio;
use crate::typedesc::TypeDesc;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

pub struct ImageBuf {
    pub buf: ffi::ImageBuf,
}

impl Drop for ImageBuf {
    fn drop(&mut self) {
        unsafe { ffi::ImageBuf_destroy(self.buf) };
    }
}

pub struct WrappedBuf<'a, T> {
    pub buf: ImageBuf,
    pub data: &'a [T],
}

impl<'a, T> std::ops::Deref for WrappedBuf<'a, T> {
    type Target = ImageBuf;

    fn deref(&self) -> &Self::Target {
        &self.buf
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

    pub fn create_with_spec(
        filename: &str,
        spec: imageio::ImageSpec,
    ) -> Result<ImageBuf, String> {
        let filename = CString::new(filename).unwrap();
        let spec = match spec {
            imageio::ImageSpec::Owned(s) => s,
            imageio::ImageSpec::Ref(s) => s,
        };
        let buf =
            unsafe { ffi::ImageBuf_create_with_spec(filename.as_ptr(), spec) };

        if buf.is_null() {
            Err(format!("Could not create ImageBuf for {:?}", filename))
        } else {
            Ok(ImageBuf { buf })
        }
    }

    pub fn create_wrapper<'a, T>(
        spec: imageio::ImageSpec,
        data: &'a [T],
    ) -> Result<WrappedBuf<'a, T>, String> {
        let spec = match spec {
            imageio::ImageSpec::Owned(s) => s,
            imageio::ImageSpec::Ref(s) => s,
        };

        let buf = unsafe {
            ffi::ImageBuf_create_with_data(
                spec,
                data.as_ptr() as *const T as *const c_void,
            )
        };

        Ok(WrappedBuf {
            buf: ImageBuf { buf },
            data,
        })
    }

    pub fn create_named_wrapper<'a, T>(
        name: &str,
        spec: imageio::ImageSpec,
        data: &'a [T],
    ) -> Result<WrappedBuf<'a, T>, String> {
        let spec = match spec {
            imageio::ImageSpec::Owned(s) => s,
            imageio::ImageSpec::Ref(s) => s,
        };

        let name = CString::new(name).unwrap();
        let buf = unsafe {
            ffi::ImageBuf_create_named_with_data(
                name.as_ptr(),
                spec,
                data.as_ptr() as *const T as *const c_void,
            )
        };

        Ok(WrappedBuf {
            buf: ImageBuf { buf },
            data,
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

    pub fn read2(
        &self,
        subimage: i32,
        miplevel: i32,
        force: bool,
    ) -> Result<(), String> {
        let result =
            unsafe { ffi::ImageBuf_read2(self.buf, subimage, miplevel, force) };
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

    pub fn name(&self) -> String {
        unsafe {
            CStr::from_ptr(ffi::ImageBuf_name(self.buf))
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn spec(&self) -> imageio::ImageSpec {
        let spec = unsafe { ffi::ImageBuf_spec(self.buf) };
        assert!(!spec.is_null());
        imageio::ImageSpec::Ref(spec)
    }

    pub unsafe fn local_pixels<T: imageio::ImageElement>(
        &self,
    ) -> Result<&[T], String> {
        let ptr = ffi::ImageBuf_localpixels(self.buf);
        if ptr.is_null() {
            return Err(
                "local pixels returned NULL. Image is not loaded.".into()
            );
        }
        let spec = self.spec();
        let num_elements = spec.width() * spec.height() * spec.nchannels();

        Ok(std::slice::from_raw_parts(
            ptr as *const T,
            num_elements as usize,
        ))
    }

    pub unsafe fn local_pixels_mut<T: imageio::ImageElement>(
        &self,
    ) -> Result<&[T], String> {
        let ptr = ffi::ImageBuf_localpixels_mut(self.buf);
        if ptr.is_null() {
            return Err(
                "local pixels returned NULL. Image is not loaded.".into()
            );
        }
        let spec = self.spec();
        let num_elements = spec.width() * spec.height() * spec.nchannels();

        Ok(std::slice::from_raw_parts_mut(
            ptr as *mut T,
            num_elements as usize,
        ))
    }

    pub fn width(&self) -> i32 {
        self.spec().width()
    }

    pub fn height(&self) -> i32 {
        self.spec().height()
    }

    pub fn nchannels(&self) -> i32 {
        self.spec().nchannels()
    }
}
