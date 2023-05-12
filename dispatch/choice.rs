mod WhichToUse{
    // * Tip:
    // * Traits objects are the right choice whenever you need a collection of values of mixed types, all together. (think salad)
    /*
    Generics have two major advantages over trait objects:

    1. Speed. 
        When the compiler generates machine code for a generic function, 
        it knows which types it's working with, so it knows at that time which write method to call. No need for dynamic dispatch. 
        Wheras with trait objects, Rust never knows what type of value a trait object points to until runtime.
    
    2. Not every trait can support trait objects.
    
    */
}