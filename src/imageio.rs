use crate::ffi;
use crate::typedesc;
use crate::typedesc::TypeDesc;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;

pub struct ImageSpec {
    spec: ffi::ImageSpec,
}

impl ImageSpec {
    pub fn new() -> ImageSpec {
        ImageSpec {
            spec: unsafe { ffi::ImageSpec_create() },
        }
    }

    pub fn with_dimensions(xres: i32, yres: i32, nchans: i32, fmt: TypeDesc) -> ImageSpec {
        ImageSpec {
            spec: unsafe { ffi::ImageSpec_create_with_dimensions(xres, yres, nchans, fmt) },
        }
    }
}

impl Drop for ImageSpec {
    fn drop(&mut self) {
        unsafe { ffi::ImageSpec_destroy(self.spec) };
    }
}

pub struct ImageOutput {
    io: ffi::ImageOutput,
}

#[repr(u32)]
pub enum OpenMode {
    Create = 0,
    AppendSubImage,
    AppendMIPLevel,
}

pub const AUTOSTRIDE: i64 = std::i64::MIN;

pub trait ImageElement {
    const BASETYPE: typedesc::BaseType;
    const AGGREGATE: typedesc::Aggregate;
    const VECSEMANTICS: typedesc::VecSemantics;
    const ARRAYLEN: i32;

    fn type_desc() -> typedesc::TypeDesc {
        TypeDesc {
            basetype: Self::BASETYPE,
            aggregate: Self::AGGREGATE,
            vecsemantics: Self::VECSEMANTICS,
            reserved: 0,
            arraylen: Self::ARRAYLEN,
        }
    }
}

impl ImageElement for f32 {
    const BASETYPE: typedesc::BaseType = typedesc::BaseType::FLOAT;
    const AGGREGATE: typedesc::Aggregate = typedesc::Aggregate::SCALAR;
    const VECSEMANTICS: typedesc::VecSemantics = typedesc::VecSemantics::NOSEMANTICS;
    const ARRAYLEN: i32 = 0;
}

impl ImageOutput {
    pub fn create(filename: &str) -> Result<ImageOutput, String> {
        let filename = CString::new(filename).unwrap();
        let io = unsafe { ffi::ImageOutput_create(filename.as_ptr()) };

        if io.is_null() {
            Err(format!("Could not create ImageOutput for {:?}", filename))
        } else {
            Ok(ImageOutput { io })
        }
    }

    pub fn open(&mut self, filename: &str, spec: ImageSpec, mode: OpenMode) -> Result<(), String> {
        let filename = CString::new(filename).unwrap();
        let success =
            unsafe { ffi::ImageOutput_open(self.io, filename.as_ptr(), spec.spec, mode as i32) };
        if success {
            Ok(())
        } else {
            Err(unsafe {
                CStr::from_ptr(ffi::ImageOutput_geterror(self.io))
                    .to_string_lossy()
                    .into_owned()
            })
        }
    }

    pub fn write_image<T: ImageElement>(
        &self,
        data: &[T],
        xstride: i64,
        ystride: i64,
        zstride: i64,
    ) -> Result<(), String> {
        let success = unsafe {
            ffi::ImageOutput_write_image(
                self.io,
                T::type_desc(),
                data.as_ptr() as *const T as *const c_void,
                xstride,
                ystride,
                zstride,
            )
        };
        if success {
            Ok(())
        } else {
            Err(unsafe {
                CStr::from_ptr(ffi::ImageOutput_geterror(self.io))
                    .to_string_lossy()
                    .into_owned()
            })
        }
    }
}

impl Drop for ImageOutput {
    fn drop(&mut self) {
        unsafe { ffi::ImageOutput_destroy(self.io) };
    }
}

#[test]
fn test_write_image() {
    let width = 128;
    let height = 128;
    let mut data = Vec::<f32>::new();

    for y in 0..height {
        for x in 0..width {
            data.push((x as f32) / (width as f32));
            data.push((y as f32) / (height as f32));
            data.push(0.0);
        }
    }

    let spec = ImageSpec::with_dimensions(width, height, 3, f32::type_desc());
    let mut io = ImageOutput::create("testimg.exr").unwrap();
    io.open("testimg.exr", spec, OpenMode::Create).unwrap();
    let scanline_size = std::mem::size_of::<f32>() * (width as usize) * 3;
    io.write_image(&data[..], AUTOSTRIDE, scanline_size as i64, AUTOSTRIDE)
        .unwrap();
}
