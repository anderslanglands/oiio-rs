#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/imagebufalgo.h>
#include <OpenImageIO/imageio.h>
#include <iostream>

typedef enum oiio_result {
    OIIO_SUCCESS,
    OIIO_OPEN_FAILED,
    OIIO_WRITE_FAILED
} oiio_result;

typedef enum OpenMode {
    Create,
    AppendSubImage,
    AppendMIPLevel,
} OpenMode;

struct TypeDesc {
    unsigned char basetype;
    unsigned char aggregate;
    unsigned char vecsemantics;
    unsigned char reserved;
    int arraylen;
};

typedef struct OIIO::ImageSpec* ImageSpec;
typedef struct OIIO::ImageOutput* ImageOutput;
typedef struct OIIO::ImageBuf* ImageBuf;

extern "C" {

ImageSpec ImageSpec_create() {
    auto spec = new OIIO::ImageSpec;
    return spec;
}

ImageSpec ImageSpec_create_with_dimensions(int xres, int yres, int nchans,
                                           TypeDesc fmt) {
    auto spec = new OIIO::ImageSpec(xres, yres, nchans, *(OIIO::TypeDesc*)&fmt);
    return spec;
}

void ImageSpec_destroy(ImageSpec spec) { delete spec; }

ImageOutput ImageOutput_create(const char* filename) {
    auto io = OIIO::ImageOutput::create(filename);
    return io.release();
}

bool ImageOutput_open(ImageOutput io, const char* filename, ImageSpec spec,
                      int mode) {
    return io->open(filename, *spec, OIIO::ImageOutput::OpenMode(mode));
}

bool ImageOutput_write_image(ImageOutput io, TypeDesc fmt, const void* data,
                             ptrdiff_t xstride, ptrdiff_t ystride,
                             ptrdiff_t zstride) {
    return io->write_image(*(OIIO::TypeDesc*)&fmt, data, xstride, ystride,
                           zstride);
}

static std::string ImageOutput_errorstr;
const char* ImageOutput_geterror(ImageOutput io) {
    ImageOutput_errorstr = io->geterror();
    return ImageOutput_errorstr.c_str();
}

void ImageOutput_destroy(ImageOutput io) { OIIO::ImageOutput::destroy(io); }

ImageBuf ImageBuf_create(const char* filename) {
    return new OIIO::ImageBuf(OIIO::string_view(filename));
}

ImageBuf ImageBuf_create_with_data(ImageSpec spec, void* data) {
    return new OIIO::ImageBuf(*spec, data);
}

void ImageBuf_destroy(ImageBuf imbuf) { delete imbuf; }

bool ImageBuf_read(ImageBuf imbuf) { return imbuf->read(); }

bool ImageBuf_write(ImageBuf imbuf, const char* filename, TypeDesc dtype) {
    return imbuf->write(filename, *(OIIO::TypeDesc*)&dtype);
}

OIIO::ImageBufAlgo::CompareResults ImageBufAlgo_compare(ImageBuf a, ImageBuf b,
                                                        float failthresh,
                                                        float warnthresh) {
    return OIIO::ImageBufAlgo::compare(*a, *b, failthresh, warnthresh);
}

ImageBuf ImageBufAlgo_colorconvert(ImageBuf src, const char* fromspace,
                                   const char* tospace) {
    return new OIIO::ImageBuf(
        OIIO::ImageBufAlgo::colorconvert(*src, fromspace, tospace));
}

ImageBuf ImageBufAlgo_absdiff(ImageBuf a, ImageBuf b) {
    return new OIIO::ImageBuf(OIIO::ImageBufAlgo::absdiff(*a, *b));
}

ImageBuf ImageBufAlgo_mulimg(ImageBuf a, ImageBuf b) {
    return new OIIO::ImageBuf(OIIO::ImageBufAlgo::mul(*a, *b));
}

ImageBuf ImageBufAlgo_mulconst(ImageBuf a, float b) {
    return new OIIO::ImageBuf(OIIO::ImageBufAlgo::mul(*a, b));
}

ImageBuf ImageBufAlgo_colormap(ImageBuf a, int srcchannel,
                               const char* mapname) {
    return new OIIO::ImageBuf(
        OIIO::ImageBufAlgo::color_map(*a, srcchannel, mapname));
}

} // extern "C"
