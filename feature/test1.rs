#[cfg(feature="ask")]
pub mod name{

    pub fn call(){
        println!("Rohitas");
    }
}

use crate::name;
fn main(){
    name::call();
}
