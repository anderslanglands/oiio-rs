use crate::ffi;
use crate::typedesc;
use crate::typedesc::TypeDesc;
use num_traits::Zero;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

pub enum ImageSpec {
    Owned(ffi::ImageSpec),
    Ref(ffi::ImageSpec),
}

impl ImageSpec {
    pub fn new() -> ImageSpec {
        ImageSpec::Owned(unsafe { ffi::ImageSpec_create() })
    }

    pub fn with_dimensions(
        xres: i32,
        yres: i32,
        nchans: i32,
        fmt: TypeDesc,
    ) -> ImageSpec {
        ImageSpec::Owned(unsafe {
            ffi::ImageSpec_create_with_dimensions(xres, yres, nchans, fmt)
        })
    }

    pub fn set_channel_names(&mut self, channel_names: &[String]) {
        let c_names = channel_names
            .iter()
            .map(|n| CString::new(n.as_str()).unwrap())
            .collect::<Vec<_>>();

        let ptr_names = c_names.iter().map(|n| n.as_ptr()).collect::<Vec<_>>();

        unsafe {
            let spec = match &self {
                ImageSpec::Owned(s) => s,
                ImageSpec::Ref(s) => s,
            };
            ffi::ImageSpec_set_channel_names(
                *spec,
                ptr_names.len() as i32,
                ptr_names.as_ptr() as *const *const c_char,
            );
        }
    }

    pub fn width(&self) -> i32 {
        let spec = match &self {
            ImageSpec::Owned(s) => s,
            ImageSpec::Ref(s) => s,
        };
        unsafe { ffi::ImageSpec_width(*spec) }
    }

    pub fn height(&self) -> i32 {
        let spec = match &self {
            ImageSpec::Owned(s) => s,
            ImageSpec::Ref(s) => s,
        };
        unsafe { ffi::ImageSpec_height(*spec) }
    }

    pub fn depth(&self) -> i32 {
        let spec = match &self {
            ImageSpec::Owned(s) => s,
            ImageSpec::Ref(s) => s,
        };
        unsafe { ffi::ImageSpec_depth(*spec) }
    }

    pub fn nchannels(&self) -> i32 {
        let spec = match &self {
            ImageSpec::Owned(s) => s,
            ImageSpec::Ref(s) => s,
        };
        unsafe { ffi::ImageSpec_nchannels(*spec) }
    }

    pub fn format(&self) -> TypeDesc {
        let spec = match &self {
            ImageSpec::Owned(s) => s,
            ImageSpec::Ref(s) => s,
        };
        unsafe { ffi::ImageSpec_format(*spec) }
    }
}

impl Drop for ImageSpec {
    fn drop(&mut self) {
        match self {
            ImageSpec::Owned(s) => {
                unsafe { ffi::ImageSpec_destroy(*s) };
            }
            ImageSpec::Ref(_) => (),
        }
    }
}

#[repr(C)]
pub struct ROI {
    xbegin: i32,
    xend: i32,
    ybegin: i32,
    yend: i32,
    zbegin: i32,
    zend: i32,
    chbegin: i32,
    chend: i32,
}

impl ROI {
    pub fn all() -> ROI {
        ROI::default()
    }

    pub fn new(xbegin: i32, xend: i32, ybegin: i32, yend: i32) -> ROI {
        ROI {
            xbegin,
            xend,
            ybegin,
            yend,
            zbegin: 0,
            zend: 1,
            chbegin: 0,
            chend: 10000,
        }
    }
}

impl Default for ROI {
    fn default() -> ROI {
        ROI {
            xbegin: std::i32::MIN,
            xend: 0,
            ybegin: 0,
            yend: 0,
            zbegin: 0,
            zend: 0,
            chbegin: 0,
            chend: 0,
        }
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

pub trait ImageElement: Zero {
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

impl ImageElement for u8 {
    const BASETYPE: typedesc::BaseType = typedesc::BaseType::UINT8;
    const AGGREGATE: typedesc::Aggregate = typedesc::Aggregate::SCALAR;
    const VECSEMANTICS: typedesc::VecSemantics =
        typedesc::VecSemantics::NOSEMANTICS;
    const ARRAYLEN: i32 = 0;
}

impl ImageElement for f32 {
    const BASETYPE: typedesc::BaseType = typedesc::BaseType::FLOAT;
    const AGGREGATE: typedesc::Aggregate = typedesc::Aggregate::SCALAR;
    const VECSEMANTICS: typedesc::VecSemantics =
        typedesc::VecSemantics::NOSEMANTICS;
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

    fn geterror(&self) -> String {
        unsafe {
            CStr::from_ptr(ffi::ImageOutput_geterror(self.io))
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn open(
        &mut self,
        filename: &str,
        spec: ImageSpec,
        mode: OpenMode,
    ) -> Result<(), String> {
        let filename = CString::new(filename).unwrap();
        let spec = match spec {
            ImageSpec::Owned(s) => s,
            ImageSpec::Ref(s) => s,
        };
        let success = unsafe {
            ffi::ImageOutput_open(self.io, filename.as_ptr(), spec, mode as i32)
        };
        if success {
            Ok(())
        } else {
            Err(self.geterror())
        }
    }

    pub unsafe fn write_image<T: ImageElement>(
        &self,
        data: &[T],
        xstride: i64,
        ystride: i64,
        zstride: i64,
    ) -> Result<(), String> {
        let success = ffi::ImageOutput_write_image(
            self.io,
            T::type_desc(),
            data.as_ptr() as *const T as *const c_void,
            xstride,
            ystride,
            zstride,
        );
        if success {
            Ok(())
        } else {
            Err(CStr::from_ptr(ffi::ImageOutput_geterror(self.io))
                .to_string_lossy()
                .into_owned())
        }
    }
}

impl Drop for ImageOutput {
    fn drop(&mut self) {
        unsafe { ffi::ImageOutput_destroy(self.io) };
    }
}

pub struct ImageInput {
    ii: ffi::ImageInput,
}

impl ImageInput {
    pub fn open(filename: &str) -> Result<ImageInput, String> {
        let filename = CString::new(filename).unwrap();
        let ii = unsafe { ffi::ImageInput_open(filename.as_ptr()) };

        if ii.is_null() {
            let errstr = unsafe {
                CStr::from_ptr(ffi::OIIO_geterror())
                    .to_string_lossy()
                    .into_owned()
            };
            Err(errstr)
        } else {
            Ok(ImageInput { ii })
        }
    }

    fn geterror(&self) -> String {
        unsafe {
            CStr::from_ptr(ffi::ImageInput_geterror(self.ii))
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn spec(&self) -> ImageSpec {
        let spec = unsafe { ffi::ImageInput_spec(self.ii) };
        assert!(!spec.is_null());
        ImageSpec::Ref(spec)
    }

    pub fn read_image<T>(&mut self) -> Result<Vec<T>, String>
    where
        T: ImageElement + Clone,
    {
        let spec = self.spec();
        let nelements =
            (spec.width() * spec.height() * spec.nchannels()) as usize;
        let mut result = vec![T::zero(); nelements];
        let success = unsafe {
            ffi::ImageInput_read_image(
                self.ii,
                T::type_desc(),
                result.as_mut_ptr() as *mut c_void,
            )
        };

        if success {
            Ok(result)
        } else {
            Err(self.geterror())
        }
    }

    pub fn close(&self) -> Result<(), String> {
        let success = unsafe { ffi::ImageInput_close(self.ii) };

        if success {
            Ok(())
        } else {
            Err(self.geterror())
        }
    }
}

impl Drop for ImageInput {
    fn drop(&mut self) {
        unsafe { ffi::ImageInput_destroy(self.ii) };
    }
}

// #[test]
// fn test_write_image() {
//     let width = 128;
//     let height = 128;
//     let mut data = Vec::<f32>::new();

//     for y in 0..height {
//         for x in 0..width {
//             data.push((x as f32) / (width as f32));
//             data.push((y as f32) / (height as f32));
//             data.push(0.0);
//         }
//     }

//     let spec = ImageSpec::with_dimensions(width, height, 3, f32::type_desc());
//     let mut io = ImageOutput::create("testimg.exr").unwrap();
//     io.open("testimg.exr", spec, OpenMode::Create).unwrap();
//     let scanline_size = std::mem::size_of::<f32>() * (width as usize) * 3;
//     unsafe {
//         io.write_image(&data[..], AUTOSTRIDE, scanline_size as i64, AUTOSTRIDE)
//             .expect("Image write failed")
//     };
// }
