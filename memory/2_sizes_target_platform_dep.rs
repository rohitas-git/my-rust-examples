fn main(){
    use std::mem::*;
    print!(" isize:{}\n usize:{}\n &i8:{}\n &u32:{}\n",
    size_of::<isize>(),
    size_of::<usize>(),
    size_of::<&i8>(),
    size_of::<&u32>());

    match size_of::<isize>() {
        x if x==8 =>println!("64-bit system"),
        x if x==4 =>println!("32-bit system"),
        _ => print!("None")
    };
}