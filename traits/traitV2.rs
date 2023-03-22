// * Traits are Rust's take on the interfaces or abstract base classes found in OOP-world.

mod DefImplTrait{
    // * Defining a trait is just a matter of giving it a name and a list of type signatures of the trait's methods.

    // * Syntax: The syntax for implementing a trait is the following: impl TraitName for Type

    mod Default{
        // Default Methods
        // Term: Methods listed within traits can have default implementations. 
        // In such cases, it's not required that a type implementing the trait explicitly define the method.
    }
}

mod TraitExample{
    // std::io::Write Condensed
    trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        // There's lots more
    }
}

mod TraitAsFnParameter{
    // Assume we wait to write a function whose parameter is a value of any type that can write to a stream. 
    // It'd look something like this:

    use std::io::Write;
    
    fn say_hello(out: &mut Write) -> std::io::Result<()> {
        out.write_all(b"hello!\n")?;
        out.flush();
    }

    // Pronunciation: The parameter of the above out function is of type &mut Write, 
    // meaning "a mutable reference to any value that implements the Write trait.
}

mod UsingTrait{
    // * A trait is a feature that any given type may or may not support. Think of a trait as a type capability.
    
    // * Rule: For trait methods to be accessible, the trait itself must be in scope! Otherwise, all of its methods are hidden.
    
    // use std::io::Write
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello!")?; // ERR: no method named write_all

}


mod TraitObject{
    // * Term: A reference to a trait type, like writer in the above code, is called a trait object.
    mod example{
        use std::io::Write;

        let mut buf: Vec<u8> = vec![];
        let writer: &mut Write = &mut buf; // OK!
    }

    mod error{

        use std::io::Write;

        let mut buf: Vec<u8> = vec![];
        let writer: Write = buf; // ERR: `Write` does not have a constant size

        // Rust doesn't allow variables of type Write (the trait) because a variable's size must be known at compile-time, 
        // and types that implement Write can be of any size.
    }

    mod Layout{
        // * Trait Object Layout
        // 
        // In memory, a trait object is a fat pointer (two words on the stack) consisting of a pointer to the value, 
        // plus a pointer to a table representing that value's type. 
        // 
        // Implicit Behavior: Rust automatically converts ordinary referencs into trait object when needed. 
        // This was the case with the writer variable in the above code.
    }
}

mod TraitBound{

    mod nonGeneric{
        use std::io::Write;
        fn say_hello(out: &mut Write) -> std::io::Result<()> {
            out.write_all(b"hello!\n")?;
            out.flush();
        }
    }

    mod Generic{
        use std::io::Write;
        fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
            out.write_all(b"hello!\n")?;
            out.flush();
        }
    }

    // Term: In the above say_hello function, the phrase <W: Write> is what makes the function generic. 
    // * W is called a type parameter. 
    // * And : Write, as mentioned earlier, is the bound.

    // *If the generic function you're calling doesn't have any arguments that provide useful clues about the type parameter's type, 
    // * you might have to spell it out using the turbofish ::<>.

    mod MultiTraits{
        // Operator: If your type parameter needs to support several traits, 
        // you can chain the needed traits together using the + operator.

        fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) { ... }
    }

    mod MultiTraitParameter{
        fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
            data: &DataSet, map: M, reduce: R,
        ) -> Results {
            ...
        }
    }

    mod WhereClause{
        // Keyword: The type parameter bounds in the above run_query function are way too long and it makes it less readable. 
        // * The where keyword allows us to move the bounds outside of the <>:
        
        fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
            where M: Mapper + Serialize,
                  R: Reducer + Serialize {
                      ...
        }
        // Shorthand: The where clause can be used anywhere bounds are permitted: generic structs, enums, type aliases, methods, etc.
    }

    mod WithLifetimeParameters{
        // * A generic function can have both lifetime parameters and type parameters. Lifetime parameters come first:
        // Return a ref to the point in `candidates` that's closest to `target`
        fn nearest<'t, 'c, P>(target: &'t P, candidates: &'c [P]) -> &'c P
            where P: MeasureDistance {
            ...
        }
    }
}

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

mod TraitAndOthersType{
    // * Rule: 
    // Rust lets you implement any trait on any type, 
    // as long as either the trait or the type is introduced in the current trait. 
    // This is called the coherence rule. It helps Rust ensure that trait implementations are unique.

    // Term: A trait that adds a single method to a type is called an extension traits.


    // Generic impl blocks can be used to add an extension trait to a whole family of types at once.

    // Add the `write_html` method to all types that implement `Write`
    use std::io::{self, Write};

    // Trait for values to which you can send HTML
    trait WriteHtml {
        fn write_html(&mut self, &HtmlDocument) -> io::Result<()>;
    }

    // Add the HTML write capability to any type that implements std:io::Write 
    impl<W: Write> WriteHtml for W {
        fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
            ...
        }
    }
}

mod SelfInTraits{
    // Traits can use the keyword Self as a type.

    pub trait Clone {
        fn clone(&self) -> Self;
    }

    // * A trait that uses the Self type is incompatible with trait objects.
    
    // ERR: the trait `Spliceable` cannot be made into an object
    fn splice_anything(left: &Spliceable, right: &Spliceable) {
        let combo = left.splice(right);
        ...
    }
}

mod InheritanceTrait{
    trait Creature: Visible {
        fn position(&self) -> (i32, i32);
        fn facing(&self) -> Direction;
        ...
    }
}

mod StaticMethods{
    trait StringSet {
        // constructor
        fn new() -> Self;
    
        // static method
        fn from_slice(strings: &[&str]) -> Self;

        // * Trait objects don't support static methods.
    }
}

mod Fully_Qualified_MethodCall{
    // Term: 
    // * A qualified method call is one that
    // * specifies the type or trait that a method is associated with. 
    // 
    // * A fully qualified method call is one that 
    // * specifies both type and trait.

    "hello".to_string()	 
    str::to_string("hello")	 
    ToString::to_string("hello")	        // qualified
    <str as ToString>::to_string("hello")	// fully qualified

    mod WhenNeeded{
        // Generally, you'll use value.method() to call a method, but occasionally you'll need a qualified method call:

        // 1. When two methods have the same name:
    }
}














/* -------------------------------------------------------------------------- */
/*                                    LINK                                    */
/* -------------------------------------------------------------------------- */

let resource =" https://alexeden.github.io/learning-rust/programming_rust/11_traits_and_generics.html ";