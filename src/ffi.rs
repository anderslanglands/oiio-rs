use crate::typedesc::TypeDesc;
use std::os::raw::{c_char, c_void};

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
    pub(crate) fn ImageBuf_create_with_data(
        spec: ImageSpec,
        data: *const c_void,
    ) -> ImageBuf;
    pub(crate) fn ImageBuf_destroy(imbuf: ImageBuf);
    pub(crate) fn ImageBuf_read(imbuf: ImageBuf) -> bool;
    pub(crate) fn ImageBuf_write(
        imbuf: ImageBuf,
        filename: *const c_char,
        dtype: TypeDesc,
    ) -> bool;

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
}
