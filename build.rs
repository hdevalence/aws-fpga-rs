use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Build the fpga_pci library along with the Rust code
    cc::Build::new()
        .include("vendor/aws-fpga/sdk/userspace/include")
        .file("vendor/aws-fpga/sdk/userspace/fpga_libs/fpga_pci/fpga_pci.c")
        .compile("fpga_pci");

    // Also generate bindings for it:
    let bindings = bindgen::Builder::default()
        .header("vendor/aws-fpga/sdk/userspace/include/fpga_pci.h")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");
}
