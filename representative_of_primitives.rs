
//takes a reference to an argument of any type, and returns an object
// that represents the sequence of bytes contained in such an object.

/// It takes a reference to a value of type `T` and returns a slice of bytes that represents the value
/// 
/// Arguments:
/// 
/// * `o`: &T - The object to convert to bytes.
/// 
/// Returns:
/// 
/// A slice of bytes.
fn as_bytes<T>(o: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            o as *const _ as *const u8,
            std::mem::size_of::<T>()
        )
    }
}
   
fn main(){    
    println!("{:?}", as_bytes(&1i8));
    println!("{:?}", as_bytes(&2i16));
    println!("{:?}", as_bytes(&3i32));
    println!("{:?}", as_bytes(&(4i64 + 5 * 256 + 6 * 256 * 256)));
    println!("{:?}", as_bytes(&'A'));
    println!("{:?}", as_bytes(&true));
    println!("{:?}", as_bytes(&&1i8));
}