/* -------------------------------- Reference ------------------------------- */
// A reference is fundamentally just a number that is the start position of some bytes in memory. 
// Its only purpose is to represent the concept of where data of a specific type exists. 
// What makes a reference different from just a number is that 
// Rust will validate the lifetime of references doesn't last longer than what it refers to 
// (otherwise we'd get an error when we used it!).

/* ------------------------------ Dereferencing ----------------------------- */
// The process of accessing/manipulating data that is being referred to by a reference (i.e. &i32) is called dereferencing.

// References are used to access/manipulate data in two ways:

// Access to the referred data during assignment of variables.
// Access to fields or methods of the referred data.

/* ------------------------------- * Operator ------------------------------- */
fn main() {
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    println!("{}", b)
}

// Memory detail:
// Because i32 is a primitive type that implements the Copy trait, the bytes of variable a on stack are copied into the bytes of variable b.

/* ------------------------------- . Operator ------------------------------- */

struct Foo {
    value: i32
}

fn main() {
    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);
}

// The . operator is used in accessing fields and methods of a reference. It works a bit more subtly.

// let f = Foo { value: 42 };
// let ref_ref_ref_f = &&&f;
// println!("{}", ref_ref_ref_f.value);
// Whoa, why didn't we need to add *** before ref_ref_ref_f? This is because the . operator automatically dereferences a sequence of references. That last line is turned into the following by the compiler automatically.

// println!("{}", (***ref_ref_ref_f).value);
