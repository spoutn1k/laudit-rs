use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=lustreapi");

    let out_dir = env::var("OUT_DIR").unwrap();
    let processed = format!("{}/lustreapi_preprocessed.h", out_dir);

    Command::new("gcc")
        .args(&["/usr/include/lustre/lustreapi.h", "-E", "-o"])
        .arg(&processed)
        .status()
        .unwrap();

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(&processed)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Disable the generation of Debug trait implementations on the
        // `hsm_copy` struct, that for some reason bindgen does not
        // generate on the struct members.
        .no_debug("hsm_copy")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("lustre.rs"))
        .expect("Couldn't write bindings!");
}
