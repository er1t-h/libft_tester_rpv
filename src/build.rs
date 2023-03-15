use std::{env, path::{PathBuf, Path}};

extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=bindgen/wrapper.h");
    println!("cargo:rerun-if-changed=bindgen/libft.h");
    println!("cargo:rerun-if-changed=bindgen/helper.h");
    println!("cargo:rerun-if-changed=utils_c_functions/list_utils_functions.c");
    println!("cargo:rerun-if-changed=utils_c_functions/striteri_functions.c");
    println!("cargo:rerun-if-changed=utils_c_functions/strmapi_functions.c");
    println!("cargo:rerun-if-changed=../libft.a");
    println!("cargo:rustc-link-search=../");
    println!("cargo:rustc-link-lib=ft");

    if !Path::new("../libft.a").exists() {
        panic!("'../libft.a' does not exist")
    }

    let bindings = bindgen::Builder::default()
        .header("bindgen/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    std::fs::create_dir_all(".tests_putnbr").unwrap();
    std::fs::create_dir_all(".tests_putendl").unwrap();
    std::fs::create_dir_all(".tests_putstr").unwrap();
    std::fs::create_dir_all(".tests_putchar").unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("utils_c_functions/striteri_functions.c")
        .file("utils_c_functions/strmapi_functions.c")
        .file("utils_c_functions/list_utils_functions.c")
        .compile("helper");
}
