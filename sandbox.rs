// Get memory location of variables
//? How is it done? Didn't understand the code

fn main(){
    
    let arr = ["hello", "brave", "new", "world"]; 
            
    match arr.iter().min() {
        Some(n) => print!("{} ", n),
        _ => (), 
    }

    match arr.iter().max() {
        Some(n) => print!("{} ", n), _ => (),
    }  // will print "brave world"
}