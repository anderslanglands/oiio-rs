use crate::typedesc::TypeDesc;
use std::os::raw::{c_char, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageSpec_api {
    _unused: [u8; 0],
}
pub type ImageSpec = *mut ImageSpec_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageOutput_api {
    _unused: [u8; 0],
}
pub(crate) type ImageOutput = *mut ImageOutput_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageInput_api {
    _unused: [u8; 0],
}
pub(crate) type ImageInput = *mut ImageInput_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImageBuf_api {
    _unused: [u8; 0],
}
pub type ImageBuf = *mut ImageBuf_api;

#[repr(C)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Ustring {
    pub ptr: *const c_char,
}

impl Ustring {
    pub fn new(s: &str) -> Ustring {
        let cs = std::ffi::CString::new(s).unwrap();
        let ptr = unsafe { ustring_create(cs.as_ptr()) };
        Ustring { ptr }
    }

    pub fn c_str(&self) -> *const c_char {
        self.ptr
    }

    pub fn length(&self) -> usize {
        unsafe { ustring_length(self.ptr) }
    }

    pub fn ustring_hash(&self) -> u64 {
        unsafe { ustring_hash(self.ptr) }
    }
}

impl ToString for Ustring {
    fn to_string(&self) -> String {
        if self.ptr.is_null() {
            "".into()
        } else {
            unsafe {
                std::ffi::CStr::from_ptr(self.ptr)
                    .to_string_lossy()
                    .to_owned()
                    .to_string()
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CompareResults {
    pub meanerror: f64,
    pub rms_error: f64,
    pub psnr: f64,
    pub maxerror: f64,
    pub maxx: i32,
    pub maxy: i32,
    pub maxz: i32,
    pub maxc: i32,
    pub nwarn: usize,
    pub nfail: usize,
    pub error: bool,
}

#[link(name = "coiio", kind = "static")]
extern "C" {

    pub fn OIIO_geterror() -> *const c_char;

    pub(crate) fn ImageSpec_create() -> ImageSpec;
    pub(crate) fn ImageSpec_create_with_dimensions(
        xres: i32,
        yres: i32,
        nchans: i32,
        fmt: TypeDesc,
    ) -> ImageSpec;
    pub(crate) fn ImageSpec_set_channel_names(
        spec: ImageSpec,
        num_channels: i32,
        channel_names: *const *const c_char,
    );
    pub(crate) fn ImageSpec_width(spec: ImageSpec) -> i32;
    pub(crate) fn ImageSpec_height(spec: ImageSpec) -> i32;
    pub(crate) fn ImageSpec_depth(spec: ImageSpec) -> i32;
    pub(crate) fn ImageSpec_nchannels(spec: ImageSpec) -> i32;
    pub(crate) fn ImageSpec_format(spec: ImageSpec) -> TypeDesc;
    pub(crate) fn ImageSpec_destroy(spec: ImageSpec);
    pub(crate) fn ImageSpec_get_int_attribute(
        spec: ImageSpec,
        name: *const c_char,
    ) -> *const i32;
    pub(crate) fn ImageSpec_get_float_attribute(
        spec: ImageSpec,
        name: *const c_char,
    ) -> *const f32;
    pub(crate) fn ImageSpec_get_string_attribute(
        spec: ImageSpec,
        name: *const c_char,
    ) -> *const c_char;

    pub(crate) fn ImageOutput_create(filename: *const c_char) -> ImageOutput;
    pub(crate) fn ImageOutput_open(
        io: ImageOutput,
        filename: *const c_char,
        spec: ImageSpec,
        mode: i32,
    ) -> bool;
    pub(crate) fn ImageOutput_write_image(
        io: ImageOutput,
        fmt: TypeDesc,
        data: *const c_void,
        xstride: i64,
        ystride: i64,
        zstride: i64,
    ) -> bool;
    pub(crate) fn ImageOutput_geterror(io: ImageOutput) -> *const c_char;
    pub(crate) fn ImageOutput_destroy(io: ImageOutput);

    pub(crate) fn ImageInput_open(filename: *const c_char) -> ImageInput;
    pub(crate) fn ImageInput_spec(ii: ImageInput) -> ImageSpec;
    pub(crate) fn ImageInput_read_image(
        ii: ImageInput,
        dtype: TypeDesc,
        pixels: *mut c_void,
    ) -> bool;
    pub(crate) fn ImageInput_close(ii: ImageInput) -> bool;
    pub(crate) fn ImageInput_destroy(ii: ImageInput);
    pub(crate) fn ImageInput_geterror(ii: ImageInput) -> *const c_char;

    pub(crate) fn ImageBuf_create(filename: *const c_char) -> ImageBuf;
    pub(crate) fn ImageBuf_create_with_spec(
        filename: *const c_char,
        spec: ImageSpec,
    ) -> ImageBuf;
    pub(crate) fn ImageBuf_create_named_with_data(
        name: *const c_char,
        spec: ImageSpec,
        data: *const c_void,
    ) -> ImageBuf;
    pub(crate) fn ImageBuf_create_with_data(
        spec: ImageSpec,
        data: *const c_void,
    ) -> ImageBuf;
    pub(crate) fn ImageBuf_destroy(imbuf: ImageBuf);
    pub(crate) fn ImageBuf_read(imbuf: ImageBuf) -> bool;
    pub(crate) fn ImageBuf_read2(
        imbuf: ImageBuf,
        subimage: i32,
        miplevel: i32,
        force: bool,
    ) -> bool;
    pub(crate) fn ImageBuf_write(
        imbuf: ImageBuf,
        filename: *const c_char,
        dtype: TypeDesc,
    ) -> bool;
    pub(crate) fn ImageBuf_name(imbuf: ImageBuf) -> *const c_char;
    pub(crate) fn ImageBuf_spec(imbuf: ImageBuf) -> ImageSpec;
    pub(crate) fn ImageBuf_localpixels(imbuf: ImageBuf) -> *const c_void;
    pub(crate) fn ImageBuf_localpixels_mut(imbuf: ImageBuf) -> *mut c_void;

    pub(crate) fn ImageBufAlgo_compare(
        a: ImageBuf,
        b: ImageBuf,
        failthresh: f32,
        warnthresh: f32,
    ) -> CompareResults;

    pub(crate) fn ImageBufAlgo_absdiff(a: ImageBuf, b: ImageBuf) -> ImageBuf;
    pub(crate) fn ImageBufAlgo_mulimg(a: ImageBuf, b: ImageBuf) -> ImageBuf;
    pub(crate) fn ImageBufAlgo_mulconst(a: ImageBuf, b: f32) -> ImageBuf;
    pub(crate) fn ImageBufAlgo_colormap(
        a: ImageBuf,
        srcchannel: i32,
        mapname: *const c_char,
    ) -> ImageBuf;

    pub(crate) fn ImageBufAlgo_colorconvert(
        src: ImageBuf,
        fromspace: *const c_char,
        tospace: *const c_char,
    ) -> ImageBuf;
    pub(crate) fn ImageBufAlgo_zero(buf: ImageBuf);
    pub(crate) fn ImageBufAlgo_resize(
        src: ImageBuf,
        filtername: *const c_char,
        filtersize: f32,
        roi: crate::imageio::ROI,
    ) -> ImageBuf;
    pub(crate) fn ImageBufAlgo_channels(
        src: ImageBuf,
        channel_order: *const i32,
        n_channel_order: i32,
    ) -> ImageBuf;

    pub(crate) fn ustring_create(s: *const c_char) -> *const c_char;
    pub(crate) fn ustring_length(s: *const c_char) -> usize;
    pub(crate) fn ustring_hash(s: *const c_char) -> u64;
}
