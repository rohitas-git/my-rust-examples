/* -------------------------------------------------------------------------- */
/*                                 Raw Pointer                                */
/* -------------------------------------------------------------------------- */

// *const T -> Immutable Raw pointer
// *mut T -> Mutable Raw pointer
// \* is part of type name
// In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

// Different from references and smart pointers, raw pointers:

// Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
// Aren’t guaranteed to point to valid memory
// Are allowed to be null
// Don’t implement any automatic cleanup

/* -------------------------------- Use Case -------------------------------- */

// - When interface with C code
// - When building up safe abstractions that the borrow checker doesn’t understand.
 

/* ---------------------------------- Code ---------------------------------- */

// Can create them in safe rust but not dereference them
mod create_raw_pointers {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}
// Because we created them directly from references guaranteed to be valid, we know these particular raw pointers are valid, 
// but we can’t make that assumption about just any raw pointer.

// Create a raw pointer to an arbitrary location in memory. 
// Trying to use arbitrary memory is undefined: 
// there might be data at that address or there might not, 
// the compiler might optimize the code so there is no memory access, 
// or the program might error with a segmentation fault.
mod uncertain_valid_raw_ptr{
    let address = 0x012345usize;
    let r = address as *const i32;
}

mod deref_raw_ptr{
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
/* 
If we instead tried to create an immutable and a mutable reference to num, the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references. 
With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race. Be careful!
*/

