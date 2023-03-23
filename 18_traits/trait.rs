use trait_rescue::*;
// use generic_fn_without_trait_bounds;

fn main(){

    println!("Quartic root of 64f32: {}", quartic_root(64f32));
    println!("Quartic root of -100f64: {}", quartic_root(-100f64));
    // println!("Quartic root of 64: {}", quartic_root(64i32))
    // println!("Quartic root: {}", quartic_root(125*5))

}

// * Traits are Rust's take on the interfaces or abstract base classes found in OOP-world.

/// * A Rust trait is a container of function signatures;
/// * The meaning of a trait is the capability to use some functions. 
/// i.e 
/// The meaning of a trait is that its function can be invoked on 
/// every type having that trait capability 
/// or as it is usually said, 
/// every type that satisfies that trait
/// All the types that implement a trait must have all its method with the same signatures. 
/// 
/// * With the "impl" statements, the capability to invoke the trait function is given to the types
/// Those "impl" statements mean that the trait is implemented here for the specified types by the specified code.
/// 
/// * "where" clause is named “trait bound”, and it is a part of the signature of the function.
/// * Function signatures are a kind of contract between 
/// the code that invokes the functions 
/// and the code in the body of the functions, 
/// and so their "where" clauses are part of that contract.
/// 
/// * For the code that invokes the function, 
/// such "where" clause means “when invoking this function, 
/// you must ensure that the type you pass for the Number type parameter implements the HasSquareRoot trait”.
/// 
/// * if the contract is seen by the code in the body of the function, 
/// that "where" clause means “when this function is invoked, 
/// the type passed for the "Number" type parameter is ensured to implement the "HasSquareRoot" trait, 
/// and so you can use every function belonging to such trait, but no other function”.
mod trait_rescue{
    pub trait HasSquareRoot{
        fn sq_root(self) -> Self;
    }
    impl HasSquareRoot for f64 {
        fn sq_root(self)->Self {f64::sqrt(self)}
    }
    impl HasSquareRoot for f32 {
        fn sq_root(self)->Self {f32::sqrt(self)}
    }
    
    pub fn quartic_root <Number> (x:Number) -> Number
    where Number: HasSquareRoot {
        x.sq_root().sq_root()
    }
}

/// In the declaration of a generic function, if there is no "where" clause, 
/// or if a type parameter is not cited in the "where" clause, 
/// no trait is associated to that type, 
/// and so you can do very little with an object of that generic type.
/// 
/// With a value of an unbounded type parameter "T", you can only:
/// • pass it as function argument, by value or by reference;
/// • return it from a function, by value or by reference;
/// • declare, initialize, or assign a local variable.
/// 
/// * Therefore, trait bounds on the type parameters of generic functions are almost always used.

mod generic_fn_without_trait_bounds{

    fn _f1<T>(a: T) -> T { a } 
    fn _f2<T>(a: T) -> T {
        let b: T = a; 
        let mut c = b;
        c = _f1(c);
        c 
    }
    fn _f3<T>(a: &T) -> &T { a }

    // Rare Case of Important Generic Standard Lib Function
    // The "swap" generic function can exchange the values of any two objects having the same type. 
    // Its signature is: "fn swap<T>(x: &mut T, y: &mut T)".
    fn _rare_case(){
        let mut a = 'A';
        let mut b = 'B';
        print!("{}, {}; ", a, b); 
        std::mem::swap(&mut a, &mut b); 
        print!("{}, {}", a, b);
    }
}

