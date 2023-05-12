/* -------------------------------------------------------------------------- */
/*              Accessing or Modifying a Mutable Static Variable              */
/* -------------------------------------------------------------------------- */

// In Rust, global variables are called static variables. 

mod immutable_static_variable{
    static HELLO_WORLD: &str = "Hello, world!";

    fn main() {
        println!("name is: {}", HELLO_WORLD);
    }
    
    // Static variables can only store references with the 'static lifetime, 
    // which means the Rust compiler can figure out the lifetime 
    // and we aren’t required to annotate it explicitly. 
    // Accessing an immutable static variable is safe.
}

/* ---------------------- Constants vs Static Variables --------------------- */
// They are quite similar

// Difference
// - Values in static variable have a fixed address in memory. 
//      Using the value will always access the same data
// - Constants are allowed to duplicate their data whenever they're used.
// 
// - Static Variables are mutable. Accessing and modifying mutable static variable is unsafe
// - Constants are immutable.

mod mutable_static_variable{
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    
    fn main() {
        add_to_count(3);
    
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    // With mutable data that is globally accessible, 
    // it’s difficult to ensure there are no data races, 
    // which is why Rust considers mutable static variables to be unsafe. 
    // Where possible, it’s preferable to use the concurrency techniques 
    // and thread-safe smart pointers we discussed in Chapter 16 so the compiler checks that 
    // data accessed from different threads is done safely.
}