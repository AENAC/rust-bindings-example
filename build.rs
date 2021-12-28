use bindgen::builder;


fn main()  
{
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=csrc/test.c");
    println!("cargo:rerun-if-changed=csrc/test.h");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("csrc/test.c")
        .compile("test.a");
    let bindings = builder().header("csrc/test.h")
    .allowlist_type("") // allow only the types listed here
    .allowlist_function("print_it") //allow only the functions listed here
    .allowlist_function("print_this_too") //allow only the functions listed here
    .allowlist_recursively(false) //and really, _only_ those
    .generate().unwrap();

    // Write the generated bindings to an output file.
    bindings.write_to_file("src/lib/bindings.rs").unwrap();
}
