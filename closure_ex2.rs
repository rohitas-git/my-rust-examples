fn main(){
    six_ways_of_closure();
}


/// Closure:
/// 
/// - Inline anonymous function
/// - with type inference
/// - single expression as body
/// - capture of any valid variable
/// 
/// Braces are required when closure contains several statements
fn six_ways_of_closure(){
    let factor = 2;
    let multiply = |a| a * factor; //1
    print!("{}", multiply(13));

    // let multiply_ref: &(Fn(i32) -> i32) = &multiply; //! -> trait objs without 'dyn' deprecated
    let multiply_ref: &dyn Fn(i32) -> i32 = &multiply;

    print!(
        " {} {} {} {} {}",
        (*multiply_ref)(13), //2
        multiply_ref(13), //3
        (|a| a * factor)(13), //4
        (|a: i32| a * factor)(13), //5
        |a| -> i32 { a * factor }(13) //6
    );
}



