// The typical alignment rule is this: 
//
// “Every object of a primitive type must have an address that is a multiple of its own size”.
//
// Therefore, such alignment requirements can create unused spaces, 
// the so-called “padding”.
//
// Padding: Unused space created due to alignment requirements 

use std::mem::*;

enum E1 { E1a, E1b }
enum E2 { E2a, E2b(f64) }

fn main(){

    println!("{} {} {} {} {} {}",
        size_of_val(&[0i16; 80]), // 80*2
        size_of_val(&(0i16, 0i64)), // 10 + 6(padding)
        size_of_val(&[(0i16, 0i64); 100]), // 16*100
        size_of_val(&E1::E1a), // 1
        size_of_val(&E2::E2a), // 8 + 7(padding)
        size_of_val(&vec![(0i16, 0i64); 100]) // only fix-sized header placed in stack (for vectors)
    );

}
