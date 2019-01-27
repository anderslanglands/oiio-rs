mod ffi;
pub mod imagebuf;
pub mod imagebufalgo;
pub mod imageio;
pub mod typedesc;

#[cfg(test)]
mod tests {
    use crate::ffi::*;
    use std::ffi::CString;
    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        let result = unsafe {
            let c_filename = CString::new("test.exr").unwrap();
            let mut pixels: Vec<f32> = Vec::new();
            pixels.resize(256 * 128 * 4, 0f32);
            for y in 0..128 {
                for x in 0..256 {
                    let idx = (y * 256 + x) * 4;
                    pixels[idx + 0] = (x as f32) / 256f32;
                    pixels[idx + 1] = (y as f32) / 128f32;
                    pixels[idx + 2] = 0f32;
                    pixels[idx + 3] = 1f32;
                }
            }
            oiio_write_image_f32(c_filename.as_ptr(), 256, 128, 4, pixels.as_slice().as_ptr());
        };
    }
}
