// #[derive(Debug, Copy)]
// struct Pt{
//     x:u32
// }

// use Values::*;
fn main() {
    let z = Box::new(5);
    let y=z.clone();
    println!("{:?}", z);
}
