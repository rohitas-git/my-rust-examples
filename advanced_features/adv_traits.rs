

/* -------------------------------------------------------------------------- */
/*                               Associated Type                              */
/* -------------------------------------------------------------------------- */
// Specifying Placeholder Types in Trait Definitions with Associated Types
mod TypePlaceholder{
    /* --------------- example of a trait with an associated type --------------- */
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    /* --------------- Diff between Generics and Associated Types --------------- */

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

        /* --------------------------- Generic Version --------------------------- */
        pub trait Iterator<T> {
            fn next(&mut self) -> Option<T>{None};
        }

        // Possible
        impl Iterator<String> for Counter{} 
        impl Iterator<u32> for Counter{} 
        impl Iterator<bool> for Counter{} 

        // *When a trait has a generic parameter, 
        // it can be implemented for a type multiple times, 
        // changing the concrete types of the generic type parameters each time.
        // *When we use the next method on Counter, 
        // we would have to provide type annotations to indicate which implementation of Iterator we want to use.

        /* ------------------------- Associated Type Version ------------------------ */

        pub trait Iterator {
            type Item;
        
            fn next(&mut self) -> Option<Self::Item>;
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.count)
            }
        }

        // *With associated types, we don’t need to annotate types 
        // because we can’t implement a trait on a type multiple times. 

    // *Associated types also become part of the trait’s contract: 
    // implementors of the trait must provide a type to stand in for the associated type placeholder. 
    // Associated types often have a name that describes how the type will be used, 
    // and documenting the associated type in the API documentation is good practice.
}

/* -------------------------------------------------------------------------- */
/*                                Default Type                                */
/* -------------------------------------------------------------------------- */
// Default Generic Type Parameters and Operator Overloading
mod DefaultType{
    // You specify a default type when declaring a generic type with the <PlaceholderType=ConcreteType> syntax.

    /* ----------------------------------- Ex ----------------------------------- */
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    // The default generic type in this code is within the Add trait.
    trait Add<Rhs=Self> {
        type Output;

        fn add(self, rhs: Rhs) -> Self::Output;
    }

    // To add Millimeters and Meters, 
    // we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // You’ll use default type parameters in two main ways:

    // To extend a type without breaking existing code
    // To allow customization in specific cases most users won’t need

    // The standard library’s Add trait is an example of the second purpose.
    // 
    // The first purpose is similar to the second but in reverse: 
    // if you want to add a type parameter to an existing trait, 
    // you can give it a default to allow extension of the functionality of the trait 
    // without breaking the existing implementation code.
}

/* -------------------------------------------------------------------------- */
/*                               Disambiguation                               */
/* -------------------------------------------------------------------------- */
// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
mod Disambiguation{
    mod methods{
        trait Pilot {
            fn fly(&self);
        }
        
        trait Wizard {
            fn fly(&self);
        }
        
        struct Human;
        
        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }
        
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }
        
        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }
    
        fn main() {
            let person = Human;
            person.fly(); // the compiler defaults to calling the method that is directly implemented on the type,
        }    
    
        fn main() {
            let person = Human;
            Pilot::fly(&person);
            Wizard::fly(&person);
            Human::fly(&person) // person.fly()
        }
    
        // Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call. 
    
        // Because the fly method takes a self parameter, if we had two types that both implement one trait, 
        // Rust could figure out which implementation of a trait to use based on the type of self.
    
        // However, associated functions that are not methods don’t have a self parameter. 
        // When there are multiple types or traits that define non-method functions with the same function name, 
        // Rust doesn't always know which type you mean unless you use fully qualified syntax.
    }
    
    // use fully qualified syntax
    mod associated_functions{
        trait Animal {
            fn baby_name() -> String;
        }
        
        struct Dog;
        
        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }
        
        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }
        
        // In main, we call the Dog::baby_name function, 
        // which calls the associated function defined on Dog directly.
        fn main() {
            println!("A baby dog is called a {}", Dog::baby_name());
        }

        // Because Animal::baby_name doesn’t have a self parameter, 
        // and there could be other types that implement the Animal trait, 
        // Rust can’t figure out which implementation of Animal::baby_name we want.
        fn main() {
            println!("A baby dog is called a {}", Animal::baby_name()); //ERROR
        }

        // To disambiguate and tell Rust that we want to use the implementation of Animal for Dog as 
        // opposed to the implementation of Animal for some other type, we need to use fully qualified syntax.
        // 
        // We’re providing Rust with a type annotation within the angle brackets, 
        // which indicates we want to call the baby_name method from the Animal trait as implemented on Dog 
        // by saying that we want to treat the Dog type as an Animal for this function call.
        fn main() {
            println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                           Fully Qualified Syntax                           */
/* -------------------------------------------------------------------------- */
// In general, fully qualified syntax is defined as follows:

    <Type as Trait>::function(receiver_if_method, next_arg, ...);

// For associated functions that aren’t methods, 
// there would not be a receiver: there would only be the list of other arguments.

/* -------------------------------------------------------------------------- */
/*                                 SuperTraits                                */
/* -------------------------------------------------------------------------- */
// Using Supertraits to Require One Trait’s Functionality Within Another Trait

mod SuperTraits{
    trait A: SuperTrait_B {}
    /* ----------------------------------- -- ----------------------------------- */
    use std::fmt;
    
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    { 
        impl OutlinePrint for Point {} // ERROR 
    }
    
    {
        use std::fmt;
    
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Newtype Pattern                              */
/* -------------------------------------------------------------------------- */
// Using the Newtype Pattern to Implement External Traits on External Types

// *Orphan rule that states we’re only allowed to implement a trait on a type 
// *if either the trait or the type are local to our crate.

// It’s possible to get around this restriction 
// *using the newtype pattern, 
// *which involves creating a new type in a tuple struct.

mod newtypePattern{
    // The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for. 
    // Then the wrapper type is local to our crate, and we can implement the trait on the wrapper.

    // *There is no runtime performance penalty for using this pattern, 
    // *and the wrapper type is elided at compile time.

    /* --------------------- example with Vec<T> and Display -------------------- */
    {
        use std::fmt;

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        fn main() {
            let w = Wrapper(vec![String::from("hello"), String::from("world")]);
            println!("w = {}", w);
        }

        // We can make a Wrapper struct that holds an instance of Vec<T>; 
        // then we can implement Display on Wrapper and use the Vec<T> value,
    }

    /* -------------------------------- Downside -------------------------------- */
    // *The downside of using this technique is that Wrapper is a new type, 
    // *so it doesn’t have the methods of the value it’s holding.
    // 
    // Solution:
        // We would have to implement all the methods of Vec<T> directly on Wrapper 
        // such that the methods delegate to self.0, 
        // which would allow us to treat Wrapper exactly like a Vec<T>
    // Solution:
        // If we wanted the new type to have every method the inner type has, 
        // implementing the Deref trait on the Wrapper to return the inner type would be a solution.
        // 

    // If we don’t want the Wrapper type to have all the methods of the inner type—for example, 
    // to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.

    // This newtype pattern is also useful even when traits are not involved. Let’s switch focus and look at some advanced ways to interact with Rust’s type system.
}