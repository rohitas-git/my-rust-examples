fn main(){
    error_handling::fallible_fn::fallible_ex3();
}


/// In Rust, there are no such things as in C++/Java - exceptions, "throw","catch", "try"; 
/// * all error-handling is based on the "Result" type, its functions, and the "match" statement.
/// 
/// * Robust Error Handling:
/// 
/// * Every function that contains an invocation to a fallible function :
/// 1. should be a fallible function       --> every invocation of a fallible function should be followed by a question mark to propagate the error condition.
/// OR
/// 2. should handle the "Result" value in a "match" statement, or similar handling.
/// 


pub mod error_handling{
    
    /// A fallible function 
    ///     invocation that return a "Result" type value.
    ///     normally returns an "Ok," but in exceptional cases it returns an "Err"
    /// 
    /// * '?'  Special Macro
    /// 
    /// The question mark is a special macro such that, 
    /// when applied to an expression like in "e?", 
    /// 
    /// if "e" is of generic type "Result<T,E>", it is expanded as the expression 
    /// "match e { Some(v) => v, _ => return e }"; 
    /// 
    /// instead, if "e" is of a generic type "Option<T>", it is expanded as the expression 
    /// "match e { Ok(v) => v, _ => return e }". 
    /// 
    /// In other words, 
    /// * ? macro examines if its argument is "Some" or "Ok", and in such case unwraps it, 
    /// * or otherwise returns it as a return value of the containing function.
    /// 
    /// * It can be applied only to expressions of type "Result<T,E>" or "Option<T>", 
    /// * and, of course, if can be used only inside a function with a proper return value type. 
    /// 
    /// If the enclosing function return value type is "Result<T1,E>", 
    /// the question mark macro can be applied only to an expression of "Result<T2,E>" type, 
    /// where "T2" can be different from "T1", but "E" must be the same; 
    /// 
    /// instead, if the enclosing function return value type is "Option<T1>", 
    /// the question mark macro can be applied only to an expression of "Option<T2>" type.
    /// 
    pub mod fallible_fn{
        

        /// Assume, as it is typical, that you are writing a function "f," which, 
        /// to accomplish its task, has to invoke several fallible functions, "f1", "f2", "f3", and "f4". 
        /// Each of them returns an error message if it fails, or a result if it is successful. 
        /// If a function fails, that error message should be immediately returned by the "f" function as its error message. 
        /// If a function is successful, its result should be passed as an argument to the next function.
        /// The result of the last function is passed as result of the "f" function.
        /// 
        pub fn fallible_ex(){
            fn f1(x: i32) -> Result<i32, String> { 
                if x == 1 {
                    Err(format!("Err. 1")) 
                } else {
                    Ok(x) 
                }
            }
            fn f2(x: i32) -> Result<i32, String> { 
                if x == 2 {
                    Err(format!("Err. 2")) 
                } else {
                    Ok(x) 
                }
            }
            fn f3(x: i32) -> Result<i32, String> {
                if x == 3 { 
                    Err(format!("Err. 3"))
                } else { 
                    Ok(x)
                } 
            }
            fn f4(x: i32) -> Result<i32, String> { 
                if x == 4 {
                    Err(format!("Err. 4")) 
                } else {
                    Ok(x) 
                }
            }
            fn f(x: i32) -> Result<i32, String> {
                match f1(x) { 
                    Ok(result) => {
                        match f2(result) { 
                            Ok(result) => {
                                match f3(result) {
                                    Ok(result) => f4(result), 
                                    Err(err_msg) => Err(err_msg),
                                } 
                            }
                            Err(err_msg) => Err(err_msg),
                        }
                    }
                    Err(err_msg) => Err(err_msg),
                }
            }
            
            match f(2) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
            
            match f(4) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
            
            match f(5) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
        }   

        /// This code can be made linear by replacing the "f" function with the following one:
        /// 
        pub fn fallible_ex2(){
            fn f1(x: i32) -> Result<i32, String> { 
                if x == 1 {
                    Err(format!("Err. 1")) 
                } else {
                    Ok(x) 
                }
            }
            fn f2(x: i32) -> Result<i32, String> { 
                if x == 2 {
                    Err(format!("Err. 2")) 
                } else {
                    Ok(x) 
                }
            }
            fn f3(x: i32) -> Result<i32, String> {
                if x == 3 { 
                    Err(format!("Err. 3"))
                } else { 
                    Ok(x)
                } 
            }
            fn f4(x: i32) -> Result<i32, String> { 
                if x == 4 {
                    Err(format!("Err. 4")) 
                } else {
                    Ok(x) 
                }
            }
            fn f(x: i32) -> Result<i32, String> {
                let result1 = f1(x);
                if result1.is_err() { return result1; } 
                let result2 = f2(result1.unwrap());
                if result2.is_err() { return result2; } 
                let result3 = f3(result2.unwrap());
                if result3.is_err() { return result3; } 
                f4(result3.unwrap())
            }
            
            match f(2) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
            
            match f(4) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
            
            match f(5) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
        }
    
        /// This pattern is so typical that a language feature has been introduced in the language. 
        /// Here is an equivalent version of the "f" function:
        /// 
        ///         fn f(x: i32) -> Result<i32, String> { f4(f3(f2(f1(x)?)?)?) }
        /// 
        pub fn fallible_ex3(){

            fn f1(x: i32) -> Result<i32, String> { 
                if x == 1 {
                    Err(format!("Err. 1")) 
                } else {
                    Ok(x) 
                }
            }
            fn f2(x: i32) -> Result<i32, String> { 
                if x == 2 {
                    Err(format!("Err. 2")) 
                } else {
                    Ok(x) 
                }
            }
            fn f3(x: i32) -> Result<i32, String> {
                if x == 3 { 
                    Err(format!("Err. 3"))
                } else { 
                    Ok(x)
                } 
            }
            fn f4(x: i32) -> Result<i32, String> { 
                if x == 4 {
                    Err(format!("Err. 4")) 
                } else {
                    Ok(x) 
                }
            }

            fn f(x: i32) -> Result<i32, String> { 
                f4(f3(f2(f1(x)?)?)?)
            }

            match f(2) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
            
            match f(4) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
            
            match f(5) {
                Ok(y) => println!("{}", y),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}