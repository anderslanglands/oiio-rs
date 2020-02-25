pub mod ffi;
pub mod imagebuf;
pub mod imagebufalgo;
pub mod imageio;
pub mod typedesc;
pub use ffi::Ustring;
pub use imageio::ImageElement;

pub use imagebuf::{ImageBuf, WrappedBuf};
pub use imageio::{ImageInput, ImageOutput, ImageSpec};
pub use typedesc::TypeDesc;

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

        unsafe {
            io.write_image(
                &data[..],
                imageio::AUTOSTRIDE,
                scanline_size as i64,
                imageio::AUTOSTRIDE,
            )
            .unwrap();
        }
    }
}
