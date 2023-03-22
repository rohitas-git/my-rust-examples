const SIZE: usize = 100_000;
const N_ARRAY: usize = 1_000_000;

// Creates an array in heap memory of 100_000 bytes
fn create_array() -> Box<[u8; SIZE]> { Box::new([0u8; SIZE]) }

// Recursively consumes SIZE*N_ARRAY bytes
fn recursive_func(n: usize) {
    let a = create_array();
    println!("{} {}", N_ARRAY - n + 1, a[0]);
    if n > 1 { recursive_func(n - 1) }
}

// turns out that if Stack_limit = 8.3*10^6 , Heap_Limit= 3980.7*10^6
// Heap_Size = 480 * Stack_Size
fn main(){
    recursive_func(N_ARRAY);
}