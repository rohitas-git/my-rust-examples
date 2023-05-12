/* -------------------------------------------------------------------------- */
/*                                     Aim                                    */
/* -------------------------------------------------------------------------- */
// Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro. 
// Rather than making our users implement the HelloMacro trait for each of their types,
// 
// A procedural macro with which users can annotate their type with #[derive(HelloMacro)] to get a default implementation of the hello_macro function.
// 
// The default implementation will print Hello, Macro! My name is TypeName! 
// where 
    // TypeName is the name of the type on which this trait has been defined.

// we’ll write a crate that enables another programmer to write code using our crate as shown below:
// 
    // use hello_macro::HelloMacro;
    // use hello_macro_derive::HelloMacro;

    // #[derive(HelloMacro)]
    // struct Pancakes;

    // fn main() {
    //     Pancakes::hello_macro();
    // }

// note: 
// we can’t yet provide the hello_macro function with default implementation that will print the name of the type 
// the trait is implemented on: Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime. 
// We need a macro to generate code at compile time.

/* -------------------------------------------------------------------------- */
/*                                    Steps                                   */
/* -------------------------------------------------------------------------- */

// At the time of this writing, procedural macros need to be in their own crate. 
// Eventually, this restriction might be lifted. 
// 
// *The convention for structuring crates and macro crates is as follows: 
// for a crate named foo, a custom derive procedural macro crate is called foo_derive
// 
// Let’s start a new crate called hello_macro_derive inside our hello_macro project:
// 
// Our two crates are tightly related, so we create the procedural macro crate within the directory of our hello_macro crate. 
// 
// If we change the trait definition in hello_macro, 
// we’ll have to change the implementation of the procedural macro in hello_macro_derive as well.

/* -------------------------------------------------------------------------- */
/*                     How to import derive macro and use                     */
/* -------------------------------------------------------------------------- */
// The two crates will need to be published separately, 
// and programmers using these crates will need to add both as dependencies and bring them both into scope. 
// 
// We could instead have the hello_macro crate use hello_macro_derive as a dependency 
// and re-export the procedural macro code. 
// However, the way we’ve structured the project makes it possible for programmers to use hello_macro even if they don’t want the derive functionality.

/* -------------------------------------------------------------------------- */
/*                                    Step                                    */
/* -------------------------------------------------------------------------- */

/* --------------------------------- Step 1 --------------------------------- */
// 1. 
// - need to declare the hello_macro_derive crate as a procedural macro crate
// - need functionality from the syn and quote crates
// 
// Therefore, Add the following to the Cargo.toml file for hello_macro_derive:
//  
    // [lib]
    // proc-macro = true

    // [dependencies]
    // syn = "1.0"
    // quote = "1.0"

/* --------------------------------- Step 2 --------------------------------- */
// 2. 
// 


/* -------------------------------------------------------------------------- */
/*                                 Stringify!                                 */
/* -------------------------------------------------------------------------- */

// println!("Hello, Macro! My name is {}!", stringify!(#name));

// The stringify! macro used here is built into Rust. 
// It takes a Rust expression, such as 1 + 2, and at compile time turns the expression into a string literal, 
// such as "1 + 2". This is different than format! or println!, macros which evaluate the expression and then turn the result into a String. 
// There is a possibility that the #name input might be an expression to print literally, so we use stringify!