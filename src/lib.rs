mod ffi;
pub mod imagebuf;
pub mod imagebufalgo;
pub mod imageio;
pub mod typedesc;
pub use imageio::ImageElement;

pub fn write_image_f32(
    filename: &str,
    width: usize,
    height: usize,
    num_channels: usize,
    flop: bool,
    data: &[f32],
) {
    if width * height * num_channels != data.len() {
        panic!(format!("Tried to write image {} {}x{}x{} ({}) but supplied data with length {}", filename, width, height, num_channels, width*height*num_channels, data.len()));
    }

    let spec = imageio::ImageSpec::with_dimensions(
        width as i32,
        height as i32,
        num_channels as i32,
        f32::type_desc(),
    );
    let mut io = imageio::ImageOutput::create(filename).unwrap();
    io.open(filename, spec, imageio::OpenMode::Create).unwrap();

    if flop {
        let scanline_size = std::mem::size_of::<f32>() * width * num_channels;
        unsafe {
            io.write_image(
                &data[(num_channels * width * (height - 1))..],
                imageio::AUTOSTRIDE,
                -(scanline_size as i64),
                imageio::AUTOSTRIDE,
            )
            .unwrap()
        };
    } else {
        let scanline_size = std::mem::size_of::<f32>() * width * num_channels;

        io.write_image(
            &data[..],
            imageio::AUTOSTRIDE,
            scanline_size as i64,
            imageio::AUTOSTRIDE,
        )
        .unwrap();
    }
}

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
            oiio_write_image_f32(
                c_filename.as_ptr(),
                256,
                128,
                4,
                pixels.as_slice().as_ptr(),
            );
        };
    }
}
