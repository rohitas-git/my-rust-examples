#[cfg(feature = "ask")]
pub mod hello{
    pub fn say(){
        println!("Hello, world!");
    }
}

// use hello; 
// #[cfg(feature = "ask)]
fn main() {
    hello::say();

    print!("Hell");
}
