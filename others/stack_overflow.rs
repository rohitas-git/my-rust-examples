const SIZE: usize = 100_000;
const N_ARRAY: usize = 1_000_000;

fn create_array()-> [u8;SIZE] {[0u8;SIZE]}
fn recursive_func(n: usize){
    let a = create_array();
    println!("{}, {}", N_ARRAY-n+1, a[0]);
    if n>1 { recursive_func(n-1) }
}

/*
    This program most probably crashes, typically emitting a message like
    “Segmentation fault”, or something similar. Indeed, it tries to allocate on the stack more
    than 100 GB of data.

    this program tries to allocate in the stack one million arrays of one hundred thousand bytes each.  
    Of course, it cannot do that, and after having
    printed some lines, it terminates, usually showing an error message, like “Segmentation fault” or 
    “Stack overflow”
*/
fn main(){
    crate::recursive_func(N_ARRAY);
}