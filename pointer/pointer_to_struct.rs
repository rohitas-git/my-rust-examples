#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct A<'text> {
    foo: &'text str,
    baz: i32,
}

fn ptr_to_struct(p: *const A) {
    let b: A = unsafe { *p }; // main part of this example!
    println!("{:#?}", b);
}

fn ptr_to_struct_2(p: usize) {
    let b = unsafe { *(p as *const A) }; // main part of this example!
    println!("{:#?}", b);
}

fn main() {
    let a = A {
        foo: "Hello",
        baz: 2000,
    };

    ptr_to_struc(&a);
    ptr_to_struct_(&a as *const _ as usize);
}