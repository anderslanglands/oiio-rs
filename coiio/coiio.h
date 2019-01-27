#ifndef COIIO_H
#define COIIO_H

// #include <stddef.h>

extern "C" {

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

ImageSpec ImageSpec_create();
ImageSpec ImageSpec_create_with_dimensions(int xres, int yres, int nchans, TypeDesc fmt);
void ImageSpec_destroy(ImageSpec spec);

ImageOutput ImageOutput_create(const char* filename);
bool ImageOutput_open(ImageOutput io, const char* filename, ImageSpec spec, int mode);
bool ImageOutput_write_image(ImageOutput io, TypeDesc fmt, const void* data,
    ptrdiff_t xstride, ptrdiff_t ystride, ptrdiff_t zstride
);
const char* ImageOutput_geterror(ImageOutput io);
void ImageOutput_destroy(ImageOutput io);

typedef struct OIIO::ImageBuf* ImageBuf;
ImageBuf ImageBuf_create(const char* filename);
void ImageBuf_destroy(ImageBuf imbuf);
bool ImageBuf_read(ImageBuf imbuf);
bool ImageBuf_write(ImageBuf imbuf, const char* filename, TypeDesc dtype);

OIIO::ImageBufAlgo::CompareResults ImageBufAlgo_compare(ImageBuf a, ImageBuf b, float failthresh, float warnthresh);



int oiio_write_image_f32(const char* filename, int width, int height, int nchannels,
              float* data);

}

#endif
