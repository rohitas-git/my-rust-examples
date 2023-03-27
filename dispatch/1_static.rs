mod static{

    mod code{
        fn do_something<T: Foo>(x: T) {
            x.method();
        }
        
        fn main() {
            let x = 5u8;
            let y = "Hello".to_string();
        
            do_something(x);
            do_something(y);
        }
    }

    mod rust_generated{
        // *Rust uses 'monomorphization' to perform static dispatch here. 
        // This means that Rust will create a special version of do_something() for both u8 and String, 
        // and then replace the call sites with calls to these specialized functions.

        fn do_something_u8(x: u8) {
            x.method();
        }
        
        fn do_something_string(x: String) {
            x.method();
        }
        
        fn main() {
            let x = 5u8;
            let y = "Hello".to_string();
        
            do_something_u8(x);
            do_something_string(y);
        }
    }

    mod upsides{
        // This has some upsides: static dispatching of any method calls, 
        // allowing for inlining and hence usually higher performance.
    }

    mod downsides{
        // It also has some downsides: causing code bloat 
        // due to many copies of the same function existing in the binary, one for each type.

        // Furthermore, compilers aren’t perfect and may “optimise” code to become slower. 
        // For example, functions inlined too eagerly will bloat the instruction cache (cache rules everything around us). 
        // *This is part of the reason that #[inline] and #[inline(always)] should be used carefully, 
        // *and one reason why using a dynamic dispatch is sometimes more efficient.

    }

    mod ponder{
        // *However, the common case is that it is more efficient to use static dispatch, 
        // and one can always have a thin statically-dispatched wrapper function that does a dynamic, 
        // but not vice versa, meaning static calls are more flexible. 
        // The standard library tries to be statically dispatched where possible for this reason.
    }
}