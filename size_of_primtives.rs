fn main(){

    use std::mem::*;
    print!("In Bytes::\n i8:{}\n u8:{}\n i16:{}\n u16:{}\n i32:{}\n u32:{}\n i64:{}\n u64:{}\n f32:{}\n f64:{}\n bool:{}\n char:{}\n",
        size_of::<i8>(),
        size_of::<u8>(),
        size_of::<i16>(),
        size_of::<u16>(),
        size_of::<i32>(),
        size_of::<u32>(),
        size_of::<i64>(),
        size_of::<u64>(),
        size_of::<f32>(),
        size_of::<f64>(),
        size_of::<bool>(),
        size_of::<char>()
    );
}