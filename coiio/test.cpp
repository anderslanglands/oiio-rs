#include "coiio.h"
#include <stdio.h>

int main(int argc, char** argv) {
    const char* filename = "test.jpg";
    int width = 256;
    int height = 128;
    int nchannels = 4;
    float* data = new float[width * height * nchannels];

    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            int i = (y * width + x) * nchannels;
            data[i + 0] = float(x)/width;
            data[i + 1] = float(y)/height;
            data[i + 2] = 0.0f;
            data[i + 3] = 1.0f;
        }
    }

    int result = oiio_write_image_f32(filename, width, height, nchannels, data);
    if (result == OIIO_OPEN_FAILED) {
        fprintf(stderr, "Could not open file '%s'", filename);
    } else if (result == OIIO_WRITE_FAILED) {
        fprintf(stderr, "Could not write to file '%s'", filename);
    }

    delete[] data;
}
