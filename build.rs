use cmake;
use std::path::Path;

pub fn main() {
    let oiio_root = std::env::var("OIIO_ROOT")
        .expect("OIIO_ROOT must be set to root of OIIO installation");
    let openexr_root = std::env::var("OPENEXR_ROOT")
        .expect("OPENEXR_ROOT must be set to root of OPENEXR installation");

    let oiio_lib_suffix = std::env::var("OIIO_LIB_SUFFIX").unwrap_or("".into());
    let openexr_lib_suffix =
        std::env::var("OPENEXR_LIB_SUFFIX").unwrap_or("".into());
    let lib_name = format!("libOpenImageIO{}.so", oiio_lib_suffix);

    let inc_oiio = Path::new(&oiio_root).join("include");
    let inc_openexr = Path::new(&openexr_root).join("include");
    let lib_oiio = Path::new(&oiio_root).join("lib").join(lib_name);

    let ilmimf_lib_name = format!("IlmImf{}.so", openexr_lib_suffix);
    let ilmimf_lib_path = Path::new(&openexr_root)
        .join("lib")
        .join(format!("lib{}", &ilmimf_lib_name));

    let iex_lib_name = format!("Iex{}.so", openexr_lib_suffix);
    let iex_lib_path = Path::new(&openexr_root)
        .join("lib")
        .join(format!("lib{}", &iex_lib_name));

    let dst_coiio = cmake::Config::new("coiio")
        .define("INC_OIIO", &inc_oiio)
        .define("LIB_OIIO", &lib_oiio)
        .define("LIB_ILMIMF", &ilmimf_lib_path)
        .define("LIB_IEX", &iex_lib_path)
        .define("INC_OPENEXR", &inc_openexr)
        .always_configure(false)
        .build();

    let openexr_lib_path = Path::new(&openexr_root).join("lib");

    println!("cargo:rustc-link-search=native={}", dst_coiio.display());
    println!(
        "cargo:rustc-link-search=native={}",
        openexr_lib_path.display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&oiio_root).join("lib").display()
    );

    // panic!(
    //     "openexr_lib_path: {}\nilmilmf_lib_name: {}\niex_lib_name: {}\nilmimf_lib_path: {}\n, iex_lib_path: {}",
    //     openexr_lib_path.display(), ilmimf_lib_name, iex_lib_name, ilmimf_lib_path.display(), iex_lib_path.display()
    // );

    println!("cargo:rustc-link-lib=static=coiio");
    println!("cargo:rustc-link-lib=dylib=OpenImageIO{}", oiio_lib_suffix);
    // println!("cargo:rustc-link-lib=dylib={}", ilmimf_lib_name);
    // println!("cargo:rustc-link-lib=dylib={}", iex_lib_name);

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
}
