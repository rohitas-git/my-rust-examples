/* -------------------------------------------------------------------------- */
/*                                  Dispatch                                  */
/* -------------------------------------------------------------------------- */

// *When code involves polymorphism, there needs to be a mechanism to determine which specific version is actually run. 
// *This is called 'dispatch.' 
// 
// *There are two major forms of dispatch: static dispatch and dynamic dispatch. 
// *While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called 'trait objects.'

mod background{
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }
    
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }
}

/* ---------------------------------- Link ---------------------------------- */
https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html
http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/raw/struct.TraitObject.html