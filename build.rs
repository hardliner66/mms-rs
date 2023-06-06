fn main() {
    #[cfg(any(feature = "cpp_api", feature = "c_api"))]
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(feature = "cpp_api")]
    cbindgen::Builder::new()
      .with_crate(&crate_dir)
      .with_language(cbindgen::Language::Cxx)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("./wrappers/cxx/bindings.hpp");

    #[cfg(feature = "c_api")]
    cbindgen::Builder::new()
      .with_crate(&crate_dir)
      .with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("./wrappers/c/bindings.h");

    #[cfg(feature = "dotnet")]
    csbindgen::Builder::default()
        .input_extern_file("src/c_api.rs")
        .csharp_namespace("mms_sharp")
        .csharp_class_name("MmsApi")
        .csharp_dll_name("MmsSharp")
        .generate_csharp_file("./wrappers/dotnet/MmsApi.cs")
        .unwrap();
}
