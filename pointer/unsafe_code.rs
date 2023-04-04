// What is an unsafe code? 
// Unsafe code behaves exactly like normal Rust 
// with the exception of a few abilities that the Rust compiler is unable to make guarantees about.

// A primary ability of unsafe code is dereferencing a raw pointer.
// That means taking a raw pointer to a position in memory and declaring "a data structure exists here!" 
// and turning it into a representation of data you can use (i.e. *const u8 into u8). 
// Rust has no way to keep track of the meaning of every byte that gets written to memory. 
// Because Rust can't make guarantees about what exists at an arbitrary number used as a raw pointer, 
// it puts the dereference in an unsafe { ... } block.

// Smart pointers dereference raw pointers extensively, but they are well proven in what they do.

fn main() {
    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    let pointer_a = &a as *const u8 as usize;
    println!("Data memory location: {}", pointer_a);
    
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let pointer_b = pointer_a as *const f32;
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_b
    };
    println!("I swear this is a pie! {}", b);
}