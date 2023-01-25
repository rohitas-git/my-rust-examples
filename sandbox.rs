// Get memory location of variables
//? How is it done? Didn't understand the code

fn main(){
    // let arr=[23, 17, 12, 16, 15, 2]; 
    // println!("{:?}", arr[2..5]); // * size of arr[2..5] unknown at compile time

    let arr = [23, 17, 12, 16, 15, 2];
    println!("{:?}", &arr[2..4]);
}