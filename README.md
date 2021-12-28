This compiles a c file to rust bindings, then builds a rust executable around those bindings. 

To adapt this for your projects, 

1. create your own `csrc`, perhaps from a git submodule?
2. Update build.rs with the list of structs, functions, and files to build
