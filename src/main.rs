use test_bindings::print_it;
use test_bindings::print_this_too;

fn main() {
    println!("Hello, world!");
    unsafe {print_it();}
    unsafe {print_this_too();}
}
