/* -------------------------------------------------------------------------- */
/*                           Macros are hygienic                           */
/* -------------------------------------------------------------------------- */

// The a that's defined by the macro is in a different context to the 
// a we provided in our invocation. 
// As such, the compiler treats them as completely different identifiers, 
// even though they have the same lexical appearance.

#[cfg(feature="unhygenic")]
pub mod error{
    #[macro_export]
    macro_rules! using_a{
        ($e:expr) => {
            {
                let a = 42i32;
                $e
            }
        }
    }
}

#[cfg(feature="hygenic")]
pub mod works;
mod works{
    #[macro_export]
    macro_rules! using_a{
        ($a:ident, $e:expr) => {
            {
                let $a = 42i32;
                $e
                // println!("Value of a/10: {}", $e);
            }
        }
    }
}
fn main(){
    
    let four = self::using_a!(a, a / 10);

    // let four = using_a!(a / 10);

    println!("{}", four);
}