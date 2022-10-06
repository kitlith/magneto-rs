// use miette::IntoDiagnostic;
// use std::env;
// use std::path::PathBuf;

// fn main() -> miette::Result<()> {
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         .header("cpp/magneto1.4.h")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         // Finish the builder and generate the bindings.
//         .generate()
//         .into_diagnostic()?;

//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .into_diagnostic()?;

//     cc::Build::new()
//         .file("cpp/magneto1.4.cpp")
//         .compile("magneto-bindgen");
    
//     Ok(())
// }
