mod dynamic{
    // *Rust provides dynamic dispatch through a feature called 'trait objects.' 

    // *Trait Objects:
    // *like &Foo or Box<Foo>, 
    // *are normal values that store a value of any type that implements the given trait, 
    // *where the precise type can only be known at runtime. 

    // A function that takes a trait object is not specialised to each of the types that implements Foo: only one copy is generated, 
    // often (but not always) resulting in less code bloat. 
    // However, this comes at the cost of requiring slower virtual function calls, 
    // and effectively inhibiting any chance of inlining and related optimisations from occurring.

    // Trait objects are both simple and complicated: their core representation and layout is quite straight-forward, 
    // but there are some curly error messages and surprising behaviours to discover.
}

mod getting_trait_object{
    // There's two similar ways to get a trait object value: 
    // -casts 
    // -coercions.

    // If T is a type that implements a trait Foo (e.g. u8 for the Foo above), 
    // then the two ways to get a Foo trait object out of a pointer to T look like:
    
    let ref_to_t: &T = ...;
    /* ----------------------------------- 1. casting ----------------------------------- */
    // `as` keyword for casting
    let cast = ref_to_t as &Foo;

    /* ------------------------------- 2. coercion ------------------------------ */
    // using a `&T` in a place that has a known type of `&Foo` will implicitly coerce:
    let coerce: &Foo = ref_to_t;

    fn also_coerce(_unused: &Foo) {}
    also_coerce(ref_to_t);

    // *These trait object coercions and casts also work for 
    // *pointers like &mut T to &mut Foo and Box<T> to Box<Foo>, 
    // but that's all at the moment. Coercions and casts are identical.

    /* ------------------------------ Type Erasure ------------------------------ */
    // This operation can be seen as "erasing" the compiler's knowledge about the specific type of the pointer, 
    // and hence trait objects are sometimes referred to "type erasure".
}

mod representation{
    // pub struct TraitObject {
    //     pub data: *mut (),
    //     pub vtable: *mut (),
    // }
    // That is, a trait object like &Foo consists of a "data" pointer and a "vtable" pointer.

    // The data pointer addresses the data (of some unknown type T) that the trait object is storing, and the vtable pointer points to 
    // the vtable ("virtual method table") corresponding to the implementation of Foo for T.
    // 
    // A vtable is essentially a struct of function pointers, pointing to the concrete piece of machine code for each method in the implementation. 
    // A method call like trait_object.method() will retrieve the correct pointer out of the vtable and then do a dynamic call of it. For example:


    mod represent{
        struct FooVtable {
            destructor: fn(*mut ()),
            size: usize,
            align: usize,
            method: fn(*const ()) -> String,
        }
        
        // u8:
        
        fn call_method_on_u8(x: *const ()) -> String {
            // the compiler guarantees that this function is only called
            // with `x` pointing to a u8
            let byte: &u8 = unsafe { &*(x as *const u8) };
        
            byte.method()
        }
        
        static Foo_for_u8_vtable: FooVtable = FooVtable {
            destructor: /* compiler magic */,
            size: 1,
            align: 1,
        
            // cast to a function pointer
            method: call_method_on_u8 as fn(*const ()) -> String,
        };
        
        
        // String:
        
        fn call_method_on_String(x: *const ()) -> String {
            // the compiler guarantees that this function is only called
            // with `x` pointing to a String
            let string: &String = unsafe { &*(x as *const String) };
        
            string.method()
        }
        
        static Foo_for_String_vtable: FooVtable = FooVtable {
            destructor: /* compiler magic */,
            // values for a 64-bit computer, halve them for 32-bit ones
            size: 24,
            align: 8,
        
            method: call_method_on_String as fn(*const ()) -> String,
        };
    }

    mod in_code{
        let a: String = "foo".to_string();
        let x: u8 = 1;
        
        // let b: &Foo = &a;
        let b = TraitObject {
            // store the data
            data: &a,
            // store the methods
            vtable: &Foo_for_String_vtable
        };
        
        // let y: &Foo = x;
        let y = TraitObject {
            // store the data
            data: &x,
            // store the methods
            vtable: &Foo_for_u8_vtable
        };
        
        // b.method();
        (b.vtable.method)(b.data);
        
        // y.method();
        (y.vtable.method)(y.data);

        // If b or y were owning trait objects (Box<Foo>), 
        // there would be a (b.vtable.destructor)(b.data) (respectively y) call when they went out of scope.
    }

}