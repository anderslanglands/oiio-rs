use cmake;
use std::env;
use std::path::Path;

pub fn main() {
    let boost_root = env::var("BOOST_ROOT").unwrap_or("".to_string());

    let dst_openexr = cmake::Config::new("external/openexr")
                        .define("BOOST_ROOT", &boost_root)
                        .define("OPENEXR_BUILD_SHARED", "OFF")
                        .define("OPENEXR_BUILD_STATIC", "ON")
                        .define("OPENEXR_BUILD_PYTHON_LIBS", "OFF")
                        .define("OPENEXR_BUILD_VIEWERS", "OFF")
                        .define("OPENEXR_BUILD_TESTS", "OFF")
                        .define("OPENEXR_BUILD_UTILS", "OFF")
                        .always_configure(false)
                        .build();

    let dst_oiio = cmake::Config::new("external/oiio")
                        .define("BOOST_ROOT", &boost_root)
                        .define("LINKSTATIC", "ON")
                        .define("BUILDSTATIC", "ON")
                        .define("OIIO_BUILD_TOOLS", "OFF")
                        .define("OIIO_BUILD_TESTS", "OFF")
                        .define("USE_OPENGL", "OFF")
                        .define("USE_QT", "OFF")
                        .define("USE_PYTHON", "OFF")
                        .define("OPENEXR_ROOT_DIR", dst_openexr.to_str().unwrap())
                        .define("ILMBASE_ROOT_DIR", dst_openexr.to_str().unwrap())
                        .define("OpenEXR_USE_STATIC_LIBS", "ON")
                        .always_configure(false)
                        .build();

    let inc_oiio = dst_oiio.join("include");
    let lib_oiio = dst_oiio.join("lib").join("libOpenImageIO.a");

    let inc_openexr = dst_openexr.join("include");
    let lib_half = dst_openexr.join("lib").join("libHalf-2_3_s.a");
    let lib_iex = dst_openexr.join("lib").join("libIex-2_3_s.a");
    let lib_ilmimf = dst_openexr.join("lib").join("libIlmImf-2_3_s.a");
    let lib_imath = dst_openexr.join("lib").join("libImath-2_3_s.a");

    let dst_coiio = cmake::Config::new("coiio")
                        .define("INC_OIIO", &inc_oiio)
                        .define("LIB_OIIO", &lib_oiio)
                        .define("INC_OPENEXR", &inc_openexr)
                        .define("LIB_HALF", &lib_half)
                        .define("LIB_IEX", &lib_iex)
                        .define("LIB_ILMIMF", &lib_ilmimf)
                        .define("LIB_IMATH", &lib_imath)
                        .build();

    println!("cargo:rustc-link-search=native={}", dst_coiio.display());
    println!("cargo:rustc-link-search=native={}", dst_oiio.join("lib").display());
    println!("cargo:rustc-link-search=native={}", dst_openexr.join("lib").display());
    println!("cargo:rustc-link-search=native={}", Path::new(&boost_root).join("lib").display());

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    println!("cargo:rustc-link-lib=static=coiio");
    println!("cargo:rustc-link-lib=static=OpenImageIO");
    println!("cargo:rustc-link-lib=static=IlmImf-2_3_s");
    println!("cargo:rustc-link-lib=static=Imath-2_3_s");
    println!("cargo:rustc-link-lib=static=Iex-2_3_s");
    println!("cargo:rustc-link-lib=static=Half-2_3_s");
    println!("cargo:rustc-link-lib=static=IlmThread-2_3_s");

    println!("cargo:rustc-link-lib=dylib=tiff");
    println!("cargo:rustc-link-lib=dylib=jpeg");
    println!("cargo:rustc-link-lib=dylib=openjp2");
    println!("cargo:rustc-link-lib=dylib=webp");
    println!("cargo:rustc-link-lib=dylib=png");
    println!("cargo:rustc-link-lib=dylib=gif");
    println!("cargo:rustc-link-lib=dylib=raw");
    println!("cargo:rustc-link-lib=dylib=z");

    println!("cargo:rustc-link-lib=dylib=boost_system");
    println!("cargo:rustc-link-lib=dylib=boost_filesystem");
    println!("cargo:rustc-link-lib=dylib=boost_thread");
}