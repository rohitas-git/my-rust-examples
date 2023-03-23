mod Constructor{
    // Every time we needed a struct object, we had to specify the values of all its fields.
    // This is contrary to data abstraction principles, according to which any type should
    // have an interface independent of its implementation.

    //In Rust, 
    // * it is enough to provide one or more methods that don't take "self" as an argument and having "Self"
    // as the return value type. Therefore, such methods are commonly named constructors.

    struct Number {
        x: f64,
    }
    impl Number {
        fn new() -> Number { Number { x: 0. } }
        fn from(x: f64) -> Number { Number { x: x } }
        fn value(&self) -> f64 { self.x }
    }

    // The "new" and "from" methods are constructors. 
    // * By convention, 
    // a constructor without arguments is named "new", 
    // and a constructor with one argument is named "from"
}