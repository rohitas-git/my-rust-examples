// Macros are hygienic 

// The a that's defined by the macro is in a different context to the 
// a we provided in our invocation. 
// As such, the compiler treats them as completely different identifiers, 
// even though they have the same lexical appearance.


macro_rules! using_a {
    ($e:expr) => {
        {
            let a = 42i32;
            $e
        }
    }
}

fn main(){
    let four = using_a!(a / 10);
}