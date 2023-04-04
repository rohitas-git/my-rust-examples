// In addition to the ability to create references to existing typed data using the & operator, 
// *Rust gives us the ability to create reference-like structs called smart pointers.

// We can think of references at a high level as a type that gives us access to another type. 
// Smart pointers are different in their behavior from normal references in that 
// they operate based on internal logic that a programmer writes. You — the programmer — are the smart part.

// *Typically smart pointers implement Deref, DerefMut, and Drop traits to 
// specify the logic of what should happen when the structure is dereferenced with * and . operators.

use std::ops::Deref;
struct TattleTell<T> {
    value: T,
}
impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} was used!", std::any::type_name::<T>());
        &self.value
    }
}
fn main() {
    let foo = TattleTell {
        value: "secret message",
    };
    // dereference occurs here immediately 
    // after foo is auto-referenced for the
    // function `len`
    println!("{}", foo.len());
}

