// References can be converted into a more primitive type called a raw pointer. 
// Much like a number, it can be copied and moved around with little restriction. 
// 
// Rust makes no assurances of the validity of the memory location it points to.

// Two kinds of raw pointers exist:

// *const T - A raw pointer to data of type T that should never change.
// *mut T - A raw pointer to data of type T that can change.

// Raw pointers can be converted to and from numbers
// *Raw pointers can access data with unsafe code

// A raw pointer in Rust is similar to a pointer in C that it represents a number that can be copied or passed around, 
// and even turned into numerical types where it can be modified as a number to do pointer math.

fn main() {
    let a = 42;
    let memory_location = &a as *const i32 as usize; // u64,u128 ...
    println!("Data is here {}", memory_location);
    let memory_location = &a as *const i32 as u64; // u64,u128 \ u32 --> loss of information
    println!("Data is here {}", memory_location);
}
