/*
Consider some smart pointers we've already seen like Vec<T> and String.

Vec<T> is a smart pointer that just owns some memory region of bytes. 
The Rust compiler has no idea what exists in these bytes. 
The smart pointer interprets what it means to grab items from the region of memory it manages, 
keeps track of where data structures within those bytes begin and end, 
and then finally dereferences a raw pointer into data structures into a nice clean ergonomic interface for us to use (e.g. my_vec[3]).

Similarly, String keeps track of a memory region of bytes, 
and programmatically restricts content written to it to always be valid utf-8 and helps dereference that memory region into a type &str.

Both these datastructures use unsafe dereferencing of raw pointers to do their job.

Memory details:

Rust has an equivalent of C's malloc using alloc and Layout for getting ahold of your own memory regions to manage.
*/

use std::alloc::{alloc, Layout};
use std::ops::Deref;

struct Pie {
    secret_recipe: usize,
}

impl Pie {
    fn new() -> Self {
        // let's ask for 4 bytes
        let layout = Layout::from_size_align(4, 1).unwrap();

        unsafe {
            // allocate and save the memory location as a number
            let ptr = alloc(layout) as *mut u8;
            // use pointer math and write a few 
            // u8 values to memory
            ptr.write(86);
            ptr.add(1).write(14);
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            Pie { secret_recipe: ptr as usize }
        }
    }
}
impl Deref for Pie {
    type Target = f32;
    fn deref(&self) -> &f32 {
        // interpret secret_recipe pointer as a f32 raw pointer
        let pointer = self.secret_recipe as *const f32;
        // dereference it into a return value &f32
        unsafe { &*pointer }
    }
}
fn main() {
    let p = Pie::new();
    // "make a pie" by dereferencing our 
    // Pie struct smart pointer
    println!("{:?}", *p);
}
