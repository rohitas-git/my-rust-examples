use std::mem::size_of_val;
fn main(){
    println!("String value size:\t{}",size_of_val(&"string".to_string())); // String size: 24
    println!("Vector value size:\t{}",size_of_val(&vec![0;1]));
}

// String:
// "String" objects occupy 24 bytes in the stack, plus a variable buffer in the heap. 
// Such buffer is not taken into account by the "size_of_val" function. 

//The "size_of_val" function requires a reference, and so the expression "size_of_val(greeting)" would be illegal.