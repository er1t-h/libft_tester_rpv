use std::{path::PathBuf, env};

extern crate bindgen;

fn main() {
	println!("cargo:rerun-if-changed=wrapper.h");
	println!("cargo:rustc-link-search=.");
	println!("cargo:rustc-link-lib=ft");

	let bindings = bindgen::Builder::default()
	.header("bindgen/wrapper.h")
	.parse_callbacks(Box::new(bindgen::CargoCallbacks))
	.generate()
	.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
