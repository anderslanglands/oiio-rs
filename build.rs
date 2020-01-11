use cmake;
use std::path::Path;

pub fn main() {
    let oiio_root = std::env::var("OIIO_ROOT")
        .expect("OIIO_ROOT must be set to root of OIIO installation");
    let openexr_root = std::env::var("OPENEXR_ROOT")
        .expect("OPENEXR_ROOT must be set to root of OPENEXR installation");

    let lib_suffix = std::env::var("OIIO_LIB_SUFFIX").unwrap_or("".into());
    let lib_name = format!("libOpenImageIO{}.so", lib_suffix);

    let inc_oiio = Path::new(&oiio_root).join("include");
    let inc_openexr = Path::new(&openexr_root).join("include");
    let lib_oiio = Path::new(&oiio_root).join("lib").join(lib_name);
    let lib_imf = Path::new(&oiio_root)
        .join("lib")
        .join("libIlmImf_sidefx.so");
    let lib_iex = Path::new(&oiio_root).join("lib").join("libIex_sidefx.so");

    let dst_coiio = cmake::Config::new("coiio")
        .define("INC_OIIO", &inc_oiio)
        .define("LIB_OIIO", &lib_oiio)
        .define("LIB_ILMIMF", &lib_imf)
        .define("LIB_IEX", &lib_iex)
        .define("INC_OPENEXR", &inc_openexr)
        .always_configure(false)
        .build();

    println!("cargo:rustc-link-search=native={}", dst_coiio.display());
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&oiio_root).join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=coiio");
    println!("cargo:rustc-link-lib=dylib=OpenImageIO{}", lib_suffix);
    println!("cargo:rustc-link-lib=dylib=IlmImf{}", lib_suffix);
    println!("cargo:rustc-link-lib=dylib=Iex{}", lib_suffix);

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
}
