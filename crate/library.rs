pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

// $ rustc --crate-type=lib rary.rs
// $ ls lib*
// library.rlib

// Libraries get prefixed with "lib", and by default they get named after their crate file, 
// but this default name can be overridden by passing the --crate-name option to rustc 
// or by using the crate_name attribute.

// A crate can be compiled into a binary or into a library. By default, rustc will produce a binary from a crate. 
// This behavior can be overridden by passing the --crate-type flag to lib.

// If some_file.rs has mod declarations in it, 
// then the contents of the module files would be inserted in places where mod declarations in the crate file are found, 
// before running the compiler over it. 
// In other words, modules do not get compiled individually, only crates get compiled.