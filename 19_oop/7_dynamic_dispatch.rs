/* -------------------------------------------------------------------------- */
/*                              Dynamic Dispatch                              */
/* -------------------------------------------------------------------------- */

mod static_dispatch{
    // SOLUTION 1 //
    fn draw_text<T>(txt: T) where T: Draw {
        txt.draw();
    }
    // SOLUTION 1 //
    fn draw_text<T>(txt: &T) where T: Draw {
        txt.draw();
    }
    // we see that static dispatch can work both with pass-by-value and with pass-by-reference.
}

mod dynamic_dispatch{
    // SOLUTION 2 //
    fn draw_text(txt: &Draw) {
        txt.draw();
    }

    //So now, instead of a generic function, 
    //we have a concrete function, which gets as argument a reference to a trait.
}

// A trait is not a type. You cannot declare a variable or a function argument having a trait as type. 
// * But a reference to a trait is a valid type. However, it is not an ordinary reference.

mod valid{
    trait Tr {}
    impl Tr for bool {}
    let _a: &Tr = &true;
}

mod invalid{
    trait Tr {}
    let _a: &Tr = &true;
}

// In general, any reference to a "T" type can be used to initialize a reference to a trait implemented by "T". 
// Passing an argument to a function is a kind of initialization, and so
// any reference to a "T" type can be passed as a function argument where a reference to a
// trait implemented by "T" is expected.

// *"&Draw" is not an ordinary pointer, but a pointer capable of choosing the right
// *method to invoke, according to the type of the referred object. 
// *This is a kind of dispatch, but it happens at runtime, and so it is a “dynamic dispatch”.