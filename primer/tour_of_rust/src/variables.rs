fn main() {
    // rust infers the type of x
    let snake_case_names = 13;
    println!("{}", snake_case_names );

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);
}

// Notice how we can assign to the same variable name multiple times. 
// This is called variable shadowing and the type can be changed for subsequent references to that name.