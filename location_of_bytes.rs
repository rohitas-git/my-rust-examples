fn main(){
    let b1 = true;
    let b2 = true;
    let b3 = false;
    print!("(Virtual) Memory location of\n b1:{}\n b2:{}\n b3:{}\n",
        &b1 as *const bool as usize,
        &b2 as *const bool as usize,
        &b3 as *const bool as usize
    );
}