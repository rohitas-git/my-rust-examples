/* -------------------------------------------------------------------------- */
/*                                  NewTypes                                  */
/* -------------------------------------------------------------------------- */
// Using the Newtype Pattern for Type Safety and Abstraction
mod NewtypePattern{
    
    //1. using newtypes to indicate units
    {
        struct Millimeters(u32);
        struct Meters(u32);
    }

    //2. use the newtype pattern to abstract away some implementation details of a type: 
    //  the new type can expose a public API that is different from the API of the private inner type.
    
    //3. Newtypes can also hide internal implementation.

    // The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details
}

/* -------------------------------------------------------------------------- */
/*                  Creating Type Synonyms with Type Aliases                  */
/* -------------------------------------------------------------------------- */
mod TypeAlias{
    type Kilometers = i32; // Kilometers is not a separate, new type.
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //1. The main use case for type synonyms is to reduce repetition.
        mod lengthy{
            let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

            fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
                // --snip--
            }
            fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
                // --snip--
            }
        }
        mod reduced{
            // thunk is a word for code to be evaluated at a later time, 
            // so it’s an appropriate name for a closure that gets stored
            type Thunk = Box<dyn Fn() + Send + 'static>;

            let f: Thunk = Box::new(|| println!("hi"));

            fn takes_long_type(f: Thunk) {
                // --snip--
            }
            fn returns_long_type() -> Thunk {
                // --snip--
            }
        }

    //2. For Result<..., Error> repetition
        mod repetitive{
            use std::fmt;
            use std::io::Error;
            
            pub trait Write {
                fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
                fn flush(&mut self) -> Result<(), Error>;
            
                fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
                fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
            }
        }
        mod better{
            type Result<T> = std::result::Result<T, std::io::Error>;
            pub trait Write {
                fn write(&mut self, buf: &[u8]) -> Result<usize>;
                fn flush(&mut self) -> Result<()>;
            
                fn write_all(&mut self, buf: &[u8]) -> Result<()>;
                fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
            }

            // The type alias helps in two ways: 
            // it makes code easier to write 
            // and it gives us a consistent interface across all of std::io. 
            // 
            // Because it’s an alias, it’s just another Result<T, E>, 
            // which means we can use any methods that work on Result<T, E> with it, 
            // as well as special syntax like the ? operator.
        }
}

/* -------------------------------------------------------------------------- */
/*                      The Never Type that Never Returns                     */
/* -------------------------------------------------------------------------- */
mod NeverType{
// *Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values.
// We prefer to call it the never type 
// because it stands in the place of the return type when a function will never return.

// *The formal way of describing the behavior of expr having ! value (ex: continue, panic, loop) 
// *is that expressions of type ! can be coerced into any other type. 

    /* -------------------------- Diverging functions. -------------------------- */
    fn bar() -> ! {
        // --snip--
    }
    // This code is read as “the function bar returns never.”
    // Functions that return never are called diverging functions.
    // *We can’t create values of the type ! so bar can never possibly return.

    /* --------------------------- Use for Never Type --------------------------- */
    // *But what use is a type you can never create values for?
    // *The formal way of describing this behavior is that expressions of type ! can be coerced into any other type.
    mod continue{
        let guess = match guess.trim().parse() {
            Ok(_) => 5,  // Return Type: i32
            Err(_) => continue,  // Return Type: ! (continue has ! value)
        };
        // How were we allowed to return a u32 from one arm and have another arm that ends with continue?
        // 
        // That is, when Rust computes the type of guess, it looks at both match arms, 
        // the former with a value of u32 
        // and the latter with a ! value. 
        // 
        // Because ! can never have a value, Rust decides that the type of guess is u32.

        // We’re allowed to end this match arm with continue because continue doesn’t return a value; 
        // instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.

    }
    mod panic{
        impl<T> Option<T> {
            pub fn unwrap(self) -> T {
                match self {
                    Some(val) => val,
                    None => panic!("called `Option::unwrap()` on a `None` value"),
                }
            }
        }
        // Rust sees that val has the type T and panic! has the type !, 
        // so the result of the overall match expression is T. 
        // This code works because panic! doesn’t produce a value; it ends the program. 
        // In the None case, we won’t be returning a value from unwrap, so this code is valid.
    }
    mod loop{
        print!("forever ");

        loop {
            print!("and ever ");
        }
        // Here, the loop never ends, so ! is the value of the expression. 
        // However, this wouldn’t be true if we included a break, 
        // because the loop would terminate when it got to the break.
    }
}

/* -------------------------------------------------------------------------- */
/*                 Dynamically Sized Types and the Sized Trait                */
/* -------------------------------------------------------------------------- */
// Rust needs to know certain details about its types, 
// such as how much space to allocate for a value of a particular type.
// 
// *Rust needs to know how much memory to allocate for any value of a particular type, 
// *and all values of a type must use the same amount of memory.

// *The concept of dynamically sized types. 
// Sometimes referred to as DSTs or unsized types, 
// these types let us write code using values whose size we can know only at runtime.
// 
// *DST - str, traits

mod DST{
    // *The golden rule of dynamically sized types is 
    // *that we must always put values of dynamically sized types behind a pointer of some kind.

    // &str, Box<str> or Rc<str>
    mod str{
        // We can’t know how long the string is until runtime, 
        // meaning we can’t create a variable of type str, 
        // nor can we take an argument of type str.

        let s1: str = "Hello there!"; //ERROR
        let s2: str = "How's it going?"; //ERROR
        // If Rust allowed us to write this code, these two str values would need to take up the same amount of space. 
        // But they have different lengths: s1 needs 12 bytes of storage and s2 needs 15. 
        // This is why it’s not possible to create a variable holding a dynamically sized type.
        // 
        // Alt: 
        // a &str is two values: the address of the str and its length.
        // As such, we can know the size of a &str value at compile time: it’s twice the length of a usize
        // That is, we always know the size of a &str, no matter how long the string it refers to is. 
    }
    // *In general, this is the way in which dynamically sized types are used in Rust: 
    // *they have an extra bit of metadata that stores the size of the dynamic information.

    // &dyn Trait or Box<dyn Trait> or Rc<dyn Trait>
    mod trait{
        // Every trait is a dynamically sized type we can refer to by using the name of the trait.
    }
}
mod Sized{
    // *To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. 
    // This trait is automatically implemented for everything whose size is known at compile time. 
    // In addition, Rust implicitly adds a bound on Sized to every generic function.

    fn generic<T>(t: T) {
        // --snip--
    }
    fn generic<T: Sized>(t: T) {
        // --snip--
    }
    // *By default, generic functions will work only on types that have a known size at compile time. 
    
    // However, you can use the following special syntax to relax this restriction:
    fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }
    // *A trait bound on ?Sized means “T may or may not be Sized” 
    // and this notation overrides the default that generic types must have a known size at compile time. 
    // The ?Trait syntax with this meaning is only available for Sized, not any other traits.

    // *Also note that we switched the type of the t parameter from T to &T. 
    // Because the type might not be Sized, we need to use it behind some kind of pointer. 
    // In this case, we’ve chosen a reference.


}