fn main(){
    methods::example::call_example();
}


/// -b Two possible syntax to invoke a function:
/// -n f(x,y) : Functional Syntax
/// -n x.f(y) : Object Oriented Syntax. Such fns are called > Methods
/// 
/// * The dot notation can be used to invoke only functions declared as implementation of a trait. 
/// Such functions are named “methods”, following the “object-oriented” terminology.
mod methods{

    /// If fn can invoked via dot notation => It can also be invoked via functional notation
    mod from_dot_to_functional{

        // print!("{},", "abcd".to_string());
        // print!("{},", std::string::ToString::to_string("abcd"));

        // print!("{},", [1, 2, 3].len());
        // print!("{:?},", <[i32]>::len(&[1, 2, 3]));

        // let mut v1 = vec![0u8; 0]; 
        // v1.push(7u8);
        // print!("{:?}; ", v1);

        // let mut v2 = vec![0u8; 0];
        // Vec::push(&mut v2, 7u8);
        // print!("{:?}", v2);

        // Notice that, when transforming the dot notation into the functional notation, 
        // the object to which the function is applied becomes an additional first argument.
        // Such first argument must be decorated by the possibly required dereference symbol (&), 
        // or mutation keyword (mut) or both, which are implicit in the dot notation.

        /*
        In addition, there is a problem of scoping. 
        In the standard library, there are several functions named to_string, len, or push. 
        Using the dot notation, the proper function is automatically chosen. 
        Instead, using the functional notation, the scope of the function must be written explicitly.
        */
    }

    /// dot notation is more convienent but 
    /// The dot notation can be used to invoke only functions 
    /// declared as implementation of a trait. 
    /// Such functions are named “methods”, following the “object-oriented” terminology.
    mod how{
        // fn double(x: i32) -> i32 { 
        //     x* 2
        // }
        // print!("{}", double(7i32));
        // print!("{}", 7i32.double());  // ILLEGAL

        // Usually, if a trait contains only one function, 
        // the trait has just that name put into Pascal-case. 
        // Following this convention, it had to be named Double.
        // 
        // Expression 7i32.double() means a shorthand for the expression i32::double(7i32).
        // trait Double {
        //     fn double(self) -> Self;
        // }
        // impl Double for i32 { 
        //     fn double(self) -> Self { self * 2 }
        // }
        // print!("{}", 7i32.double());
            
    }

    pub mod example{
        pub trait LettersCount {
            fn letters_count(&self, ch: char) -> usize;
        }
        impl LettersCount for str {
            fn letters_count(&self, ch: char) -> usize { 
                let mut count = 0;
                for c in self.chars() {
                    if c == ch { count += 1;} 
                }
                count   
            }   
        }

        pub fn call_example(){
            print!("{} ", "".letters_count('a'));
            print!("{} ", "ddd".letters_count('a'));
            print!("{} ", "ddd".letters_count('d'));
            print!("{} ", "foobarbaz".letters_count('a'));
        }
    }

}