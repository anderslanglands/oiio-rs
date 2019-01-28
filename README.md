[![Build Status](https://travis-ci.com/anderslanglands/oiio-rs.svg?branch=master)](https://travis-ci.com/anderslanglands/oiio-rs)

# Dependencies
- boost-filesystem
- boost-system
- boost-thread
- tiff
- jpeg
- raw
- gif
- png
- webp
- openjp2

```yaml
before_install:
  - sudo apt-get update
  - sudo apt-get install -y libboost-dev libboost-filesystem-dev libboost-system-dev libboost-thread-dev libraw-dev libtiff-dev libjpeg-dev libopenjp2-7-dev libgif-dev libpng-dev libwebp-dev
```

# Building
Tested on OSX and Ubuntu 16.04. Must have Boost (1.61 or compatible) installed already. Can specify path to Boost using BOOST_ROOT environment variable, as in:
```bash
env BOOST_ROOT=/path/to/boost cargo build
```