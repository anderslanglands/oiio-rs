use crate::typedesc::TypeDesc;
use std::os::raw::{c_char, c_float, c_int, c_void};

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
pub(crate) enum OiioResult {
    Success,
    OpenFailed,
    WriteFailed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageSpec_api {
    _unused: [u8; 0],
}
pub(crate) type ImageSpec = *mut ImageSpec_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageOutput_api {
    _unused: [u8; 0],
}
pub(crate) type ImageOutput = *mut ImageOutput_api;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ImageBuf_api {
    _unused: [u8; 0],
}
pub(crate) type ImageBuf = *mut ImageBuf_api;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CompareResults {
    pub meanerror: f64,
    pub rms_error: f64,
    pub PSNR: f64,
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
    pub(crate) fn ImageSpec_create() -> ImageSpec;
    pub(crate) fn ImageSpec_create_with_dimensions(
        xres: i32,
        yres: i32,
        nchans: i32,
        fmt: TypeDesc,
    ) -> ImageSpec;
    pub(crate) fn ImageSpec_destroy(spec: ImageSpec);

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

    pub(crate) fn ImageBuf_create(filename: *const c_char) -> ImageBuf;
    pub(crate) fn ImageBuf_destroy(imbuf: ImageBuf);
    pub(crate) fn ImageBuf_read(imbuf: ImageBuf) -> bool;
    pub(crate) fn ImageBuf_write(imbuf: ImageBuf, filename: *const c_char, dtype: TypeDesc)
        -> bool;

    pub(crate) fn ImageBufAlgo_compare(
        a: ImageBuf,
        b: ImageBuf,
        failthresh: f32,
        warnthresh: f32,
    ) -> CompareResults;

    pub(crate) fn oiio_write_image_f32(
        filename: *const c_char,
        width: c_int,
        height: c_int,
        nchannels: c_int,
        data: *const c_float,
    ) -> OiioResult;
}
