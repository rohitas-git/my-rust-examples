fn main(){
    let b1 = true;
    let b2 = 6i16;
    let b3 = 16f64;
    print!("(Virtual) Memory location of\n b1:{}\n b2:{}\n b3:{}\n",
        &b1 as *const bool as usize,
        &b2 as *const i16 as usize,
        &b3 as *const f64 as usize
    );
}