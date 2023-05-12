fn main(){

    use std::mem::*;
    print!("In Bytes::\n i8:{}\n u8:{}\n i16:{}\n u16:{}\n i32:{}\n u32:{}\n i64:{}\n u64:{}\n f32:{}\n f64:{}\n bool:{}\n char:{}\n",
        size_of::<i8>(), //1
        size_of::<u8>(), //1
        size_of::<i16>(),//2
        size_of::<u16>(),//2
        size_of::<i32>(),//4
        size_of::<u32>(),//4
        size_of::<i64>(),//8
        size_of::<u64>(),//8
        size_of::<f32>(),//4
        size_of::<f64>(),//8
        size_of::<bool>(),//1
        size_of::<char>()//4
    );
}