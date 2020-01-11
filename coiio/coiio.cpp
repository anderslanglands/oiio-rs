#include <OpenImageIO/imagebuf.h>
#include <OpenImageIO/imagebufalgo.h>
#include <OpenImageIO/imageio.h>
#include <OpenImageIO/ustring.h>
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
typedef struct OIIO::ImageInput* ImageInput;
typedef struct OIIO::ImageOutput* ImageOutput;
typedef struct OIIO::ImageBuf* ImageBuf;
typedef struct OIIO::ustring ustring;
typedef struct OIIO::ROI ROI;

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

int ImageSpec_width(ImageSpec spec) { return spec->width; }
int ImageSpec_height(ImageSpec spec) { return spec->height; }
int ImageSpec_depth(ImageSpec spec) { return spec->depth; }
int ImageSpec_nchannels(ImageSpec spec) { return spec->nchannels; }
TypeDesc ImageSpec_format(ImageSpec spec) { return *(TypeDesc*)&spec->format; }

void ImageSpec_destroy(ImageSpec spec) { delete spec; }

void ImageSpec_set_channel_names(ImageSpec spec, int num_channels,
                                 const char** channel_names) {
    std::vector<std::string> names;
    names.reserve(num_channels);
    for (int i = 0; i < num_channels; ++i) {
        names.emplace_back(channel_names[i]);
    }
    spec->channelnames = names;
}

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

static std::string ImageInput_errorstr;
const char* ImageInput_geterror(ImageInput ii) {
    ImageInput_errorstr = ii->geterror();
    return ImageInput_errorstr.c_str();
}

static std::string OIIO_errorstr;
const char* OIIO_geterror() {
    OIIO_errorstr = OIIO::geterror();
    return OIIO_errorstr.c_str();
}

void ImageOutput_destroy(ImageOutput io) { OIIO::ImageOutput::destroy(io); }

ImageInput ImageInput_open(const char* filename) {
    auto ii = OIIO::ImageInput::open(filename);
    return ii.release();
}

ImageSpec ImageInput_spec(ImageInput ii) {
    const OIIO::ImageSpec& spec = ii->spec();
    return (ImageSpec)&spec;
}

bool ImageInput_read_image(ImageInput ii, TypeDesc fmt, void* pixels) {
    return ii->read_image(*(OIIO::TypeDesc*)&fmt, pixels);
}

bool ImageInput_close(ImageInput ii) { return ii->close(); }

void ImageInput_destroy(ImageInput ii) { OIIO::ImageInput::destroy(ii); }

ImageBuf ImageBuf_create(const char* filename) {
    return new OIIO::ImageBuf(OIIO::string_view(filename));
}

ImageBuf ImageBuf_create_with_spec(const char* filename, ImageSpec spec) {
    return new OIIO::ImageBuf(OIIO::string_view(filename), *spec);
}

ImageBuf ImageBuf_create_with_data(ImageSpec spec, void* data) {
    return new OIIO::ImageBuf(*spec, data);
}

ImageBuf ImageBuf_create_named_with_data(const char* name, ImageSpec spec,
                                         void* data) {
    return new OIIO::ImageBuf(name, *spec, data);
}

void ImageBuf_destroy(ImageBuf imbuf) { delete imbuf; }

bool ImageBuf_read(ImageBuf imbuf) { return imbuf->read(); }
bool ImageBuf_read2(ImageBuf imbuf, int subimage, int miplevel, bool force) {
    return imbuf->read(subimage, miplevel, force);
}

bool ImageBuf_write(ImageBuf imbuf, const char* filename, TypeDesc dtype) {
    return imbuf->write(filename, *(OIIO::TypeDesc*)&dtype);
}

const char* ImageBuf_name(ImageBuf imbuf) { return imbuf->name().c_str(); }

ImageSpec ImageBuf_spec(ImageBuf imbuf) {
    const OIIO::ImageSpec& spec = imbuf->spec();
    return (ImageSpec)&spec;
}

const void* ImageBuf_localpixels(ImageBuf imbuf) {
    return imbuf->localpixels();
}

void* ImageBuf_localpixels_mut(ImageBuf imbuf) { return imbuf->localpixels(); }

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

void ImageBufAlgo_zero(ImageBuf b) { OIIO::ImageBufAlgo::zero(*b); }

ImageBuf ImageBufAlgo_resize(ImageBuf src, const char* filtername,
                             float filtersize, ROI roi) {
    return new OIIO::ImageBuf(
        OIIO::ImageBufAlgo::resize(*src, filtername, filtersize, roi));
}

ImageBuf ImageBufAlgo_channels(ImageBuf src, int nchannels, int* channel_order,
                               int n_channel_order) {
    return new OIIO::ImageBuf(OIIO::ImageBufAlgo::channels(
        *src, n_channel_order,
        OIIO::cspan<int>(channel_order, n_channel_order)));
}

const char* ustring_create(const char* str) {
    return OIIO::ustring(str).c_str();
}

size_t ustring_length(const char* us) {
    return ((OIIO::ustring*)&us)->length();
}

size_t ustring_hash(const char* us) { return ((OIIO::ustring*)&us)->hash(); }

} // extern "C"
