use crate::ffi;
pub use crate::ffi::CompareResults;
use crate::imagebuf::ImageBuf;
use crate::imageio::ROI;
use std::ffi::CString;

pub fn compare(
    a: &ImageBuf,
    b: &ImageBuf,
    failthresh: f32,
    warnthresh: f32,
) -> CompareResults {
    unsafe { ffi::ImageBufAlgo_compare(a.buf, b.buf, failthresh, warnthresh) }
}

pub fn absdiff(a: &ImageBuf, b: &ImageBuf) -> ImageBuf {
    unsafe {
        ImageBuf {
            buf: ffi::ImageBufAlgo_absdiff(a.buf, b.buf),
        }
    }
}

pub fn mul_img(a: &ImageBuf, b: &ImageBuf) -> ImageBuf {
    unsafe {
        ImageBuf {
            buf: ffi::ImageBufAlgo_mulimg(a.buf, b.buf),
        }
    }
}

pub fn mul_const(a: &ImageBuf, b: f32) -> ImageBuf {
    unsafe {
        ImageBuf {
            buf: ffi::ImageBufAlgo_mulconst(a.buf, b),
        }
    }
}

pub fn zero(buf: &ImageBuf) {
    unsafe {
        ffi::ImageBufAlgo_zero(buf.buf);
    }
}

pub fn resize(
    src: &ImageBuf,
    filtername: &str,
    filtersize: f32,
    roi: ROI,
) -> ImageBuf {
    unsafe {
        let filtername = CString::new(filtername).unwrap();
        ImageBuf {
            buf: ffi::ImageBufAlgo_resize(
                src.buf,
                filtername.as_ptr(),
                filtersize,
                roi,
            ),
        }
    }
}

pub fn channels(src: &ImageBuf, channel_order: &[i32]) -> ImageBuf {
    unsafe {
        ImageBuf {
            buf: ffi::ImageBufAlgo_channels(
                src.buf,
                channel_order.as_ptr(),
                channel_order.len() as i32,
            ),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ColorMap {
    Inferno,
    Viridis,
    Magma,
    Plasma,
    BlueRed,
    Spectrum,
    Heat,
}

pub fn colormap(a: &ImageBuf, srcchannel: i32, map: ColorMap) -> ImageBuf {
    let s = match map {
        ColorMap::Inferno => CString::new("inferno").unwrap(),
        ColorMap::Viridis => CString::new("viridis").unwrap(),
        ColorMap::Magma => CString::new("magma").unwrap(),
        ColorMap::Plasma => CString::new("plasma").unwrap(),
        ColorMap::BlueRed => CString::new("blue-red").unwrap(),
        ColorMap::Spectrum => CString::new("spectrum").unwrap(),
        ColorMap::Heat => CString::new("heat").unwrap(),
    };
    unsafe {
        ImageBuf {
            buf: ffi::ImageBufAlgo_colormap(a.buf, srcchannel, s.as_ptr()),
        }
    }
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
        }
    }
}
