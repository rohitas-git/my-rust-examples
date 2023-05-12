mod Inherent_Implementation{

    mod Solution1{
        trait Tr {
            fn f1(a: u32) -> bool;
            fn f2(&self, b: u16) -> Self;
        }
        struct Stru {
            x: u16,
            y: u16,
        }

        impl Tr for Stru {
            fn f1(a:u32)->bool{ 0*a as bool}
            fn f2(&self, b:u16)->Self{Str{x:self.x, y:b}}
        }
    }

    mod Solution_OOP{
        struct Stru {
            x: u16,
            y: u16,
        }
        impl Stru {
            fn f1(a:u32)->bool{ 0*a as bool}
            fn f2(&self, b:u16)->Self{Str{x:self.x, y:b}}
        }
        
        // Now the impl statement is applied directly to a type, with no need to have a trait. 
        // Instead of implementing a trait for a type, now that type has a so-called “inherent” implementation.

        // we have a user-defined type, Stru, with some data members, x and y; and some methods, f1 and f2.
    }
}

mod OOP_RUST{
    // *struct -> Data Definitions
    // *impl construct -> Func Declaration

    // In particular, the Rust methods whose argument list begins with a self pseudo-­
    // argument are the so-called “object methods” in object-oriented programming, which in
    // C++ are named “non-static member functions”, while the Rust methods whose argument
    // list does not begin with a self pseudo-argument are the so-called “class methods” in
    // object-oriented programming, which in C++ are named “static member functions”.

    // To invoke a method having the self argument, dot notation is used, like in s.f2(456), 
    // while to invoke a method without the self argument, the syntax is like Stru::f1(500_000), 
    // that is, it requires the name of the type followed by a double-colon,
    // followed by the name of the function.

    // A difference between Rust and C++ is that when referring to a field of the current
    // object, say x, in Rust you must write self.x, while in C++ and other languages the
    // corresponding expression is this->x, but, if there is no ambiguity, you may write simply
    // x, with the same meaning.

    // Another difference between Rust and most other object oriented languages is that in
    // those languages the reference to the current object (named self, this, or Me) is always
    // a pointer or a reference. 
    // * In Rust, if you write &self in the method signature, you get a reference, while if you write simply self you get a copy of the current object.
}

mod Peculiarities{

    mod NoOrder{
        S::f2();
        impl S { fn f1() { print!("1"); } }
        impl S { }
        S::f1();
        impl S { fn f2() { print!("2"); } fn _f3() {} }
        struct S {}
    }
    // The struct and impl construct can be placed in any point and in any order, 
    // provided they are in the same scope. The struct and the function can be defined after they are used.

    mod SameNameFunc{
        struct S1 {}
        struct S2 {}
        impl S1 {
            fn f() {}
            //ILLEGAL: fn f(a: i32) {}
        }
        impl S2 {
            fn f() {}
        }
        S1::f();
        S2::f();
    }
    // In Rust you cannot have several functions having the same name in the same
    // scope. A type creates a scope. Therefore, you cannot have two methods named f for the
    // same type S1, even if such methods have different arguments. Yet, you can declare two
    // methods having the same name for two distinct types, like S1::f and S2::f.

    mod Impl_EnumOther{
        enum Continent {
            Africa,
            America,
            Asia,
            Europe,
            Oceania,
        }
        impl Continent {
            fn name(&self) -> &str {
                match *self {
                    Continent::Africa => "Africa",
                    Continent::America => "America",
                    Continent::Asia => "Asia",
                    Continent::Europe => "Europe",
                    Continent::Oceania => "Oceania",
                }
            }
        }

        // In Rust, you can add methods not only to structs, but also to any type defined in your
        // code, like enums and tuple-structs.

        // Regarding primitive types, you cannot directly add methods to them.
        impl i32 {}

        // And you cannot directly add methods to non-primitive types that are defined in the
        // standard library or in another library.
        impl Vec<i32> {}

        // Regarding traits, you cannot implement a trait declared in the standard library or in
        // another library for a type declared in the standard library or in another library.
        impl std::iter::Iterator for i32 {}

        // However, any trait defined in your code can be implemented for any type.
        trait Tr {}
        impl Tr for bool{}

        struct Pair(u32, u32);
        impl std::iter::Iterator for Pair
    }

    mod Summary{
        // Summarizing, 
        // * if "Ty" is a type, the clause "impl Ty" is allowed 
        // only if "Ty" is declared in the current crate; 
        // 
        // and, as shown by the following table, if "Tr" is a trait, the
        // *clause "impl Tr for Ty" is allowed 
        // if "Tr" or "Ty" is declared in the current crate, 
        // and it is not allowed only if both "Tr" and "Ty" are part of the language, the standard library or another library.
    }
    
}