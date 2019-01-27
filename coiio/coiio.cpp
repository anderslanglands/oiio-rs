#include <OpenImageIO/imageio.h>
#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/imagebufalgo.h>
#include "coiio.h"
#include <iostream>

extern "C" {

ImageSpec ImageSpec_create() {
    auto spec = new OIIO::ImageSpec;
    return spec;
}

ImageSpec ImageSpec_create_with_dimensions(int xres, int yres, int nchans, TypeDesc fmt) {
    auto spec = new OIIO::ImageSpec(xres, yres, nchans, *(OIIO::TypeDesc*)&fmt);
    return spec;
}

void ImageSpec_destroy(ImageSpec spec) {
    delete spec;
}

ImageOutput ImageOutput_create(const char* filename) {
    auto io = OIIO::ImageOutput::create(filename);
    return io.release();
}

bool ImageOutput_open(ImageOutput io, const char* filename, ImageSpec spec, int mode) {
    return io->open(filename, *spec, OIIO::ImageOutput::OpenMode(mode));
}

bool ImageOutput_write_image(ImageOutput io, TypeDesc fmt, const void* data,
    ptrdiff_t xstride, ptrdiff_t ystride, ptrdiff_t zstride
) {
    return io->write_image(*(OIIO::TypeDesc*)&fmt, data, xstride, ystride, zstride);
}

static std::string ImageOutput_errorstr;
const char* ImageOutput_geterror(ImageOutput io) {
    ImageOutput_errorstr = io->geterror();
    return ImageOutput_errorstr.c_str();
}

void ImageOutput_destroy(ImageOutput io) {
    OIIO::ImageOutput::destroy(io);
}

ImageBuf ImageBuf_create(const char* filename) {
    return new OIIO::ImageBuf(OIIO::string_view(filename));
}

void ImageBuf_destroy(ImageBuf imbuf) {
    delete imbuf;
}

bool ImageBuf_read(ImageBuf imbuf) {
    return imbuf->read();
}

bool ImageBuf_write(ImageBuf imbuf, const char* filename, TypeDesc dtype) {
    return imbuf->write(filename, *(OIIO::TypeDesc*)&dtype);
}

OIIO::ImageBufAlgo::CompareResults ImageBufAlgo_compare(ImageBuf a, ImageBuf b, float failthresh, float warnthresh) {
    return OIIO::ImageBufAlgo::compare(*a, *b, failthresh, warnthresh);
}

int oiio_write_image_f32(const char* filename, int width, int height, int nchannels,
              float* data) {
    auto img_out = OIIO::ImageOutput::create(filename);
    auto scanline_size = width * sizeof(float) * nchannels;
    if (!img_out->open(filename, OIIO::ImageSpec(width, height, nchannels,
                                                 OIIO::TypeDesc::FLOAT))) {
        return OIIO_OPEN_FAILED;
    }
    if (!img_out->write_image(
            OIIO::TypeDesc::FLOAT,
            reinterpret_cast<char*>(data) + (height - 1) * scanline_size,
            OIIO::AutoStride, -scanline_size, OIIO::AutoStride)) {
        return OIIO_WRITE_FAILED;
    }

    return OIIO_SUCCESS;
}
}
