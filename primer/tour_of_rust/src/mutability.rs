// Rust cares a great deal about what variables are modifiable. Values fall into two types:
// 
// mutable - the compiler will allow the variable to be written to and read from.
// immutable - the compiler will only allow the variable to be read from.

// Mutable values are denoted with a mut keyword.

fn main() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}