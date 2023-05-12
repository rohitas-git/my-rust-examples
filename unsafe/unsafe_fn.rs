/* -------------------------------------------------------------------------- */
/*                    Calling an Unsafe Function or Method                    */
/* -------------------------------------------------------------------------- */

mod example{
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}
// The unsafe keyword in this context indicates the function has requirements we need to uphold when we call this function, 
// because Rust can’t guarantee we’ve met these requirements. 
// By calling an unsafe function within an unsafe block, 
// we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.

// Bodies of unsafe functions are effectively unsafe blocks, 
// so to perform other unsafe operations within an unsafe function, 
// we don’t need to add another unsafe block.

/* -------------------------------------------------------------------------- */
/*                Creating a Safe Abstraction over Unsafe Code                */
/* -------------------------------------------------------------------------- */

mod safe_wrap_over_unsafe{
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; 
// it only knows that we’re borrowing from the same slice twice.
mod impl_with_err{
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
    
        assert!(mid <= len);
    
        (&mut values[..mid], &mut values[mid..])
    }    
}

// The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and must trust that this pointer is valid. 
// The add method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer.
mod how_it_impl{
    use std::slice;

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();
    
        assert!(mid <= len);
    
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }   
}
// WHY OK?
// Note that we don’t need to mark the resulting split_at_mut function as unsafe, 
// and we can call this function from safe Rust. 
// We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses unsafe code in a safe way, 
// because it creates only valid pointers from the data this function has access to.

mod crash_it{
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // We don’t own the memory at this arbitrary location, 
    // and there is no guarantee that the slice this code creates contains valid i32 values. 
    // Attempting to use values as though it’s a valid slice results in undefined behavior.
}

/* -------------------------------------------------------------------------- */
/*                Using extern Functions to Call External Code                */
/* -------------------------------------------------------------------------- */
// Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI).
// Functions declared within extern blocks are always unsafe to call from Rust code. 
// The reason is that other languages don’t enforce Rust’s rules and guarantees, 
// and Rust can’t check them, so responsibility falls on the programmer to ensure safety.

mod FFI_in_rust{
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    
    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
    // The "C" part defines which application binary interface (ABI) the external function uses: 
    //      the ABI defines how to call the function at the assembly level. 
    // The "C" ABI is the most common and follows the C programming language’s ABI.
}

/* --------------- Calling Rust Functions from Other Languages -------------- */
// We can also use extern to create an interface that allows other languages to call Rust functions.

mod FFI_for_rust_to_others{
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // In the following example, we make the call_from_c function accessible from C code, 
    // after it’s compiled to a shared library and linked from C:
    // This usage of extern does not require unsafe.

}

