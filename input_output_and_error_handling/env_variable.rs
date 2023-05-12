fn main(){
    // environment_variables::in_os();  // will print all the env vars in current system :)
    environment_variables::set_var2();
}

// Another form of input/output is through environment variables.
pub mod environment_variables{

    pub fn in_os(){
        for var in std::env::vars() { 
            println!("[{}]=[{}]", var.0, var.1);
        }
    }

    pub fn set_var(){
        print!("[{:?}]", std::env::var("abcd")); // Err(NotPresent)
        std::env::set_var("abcd", "This is the value"); // Set the environment variable
        print!(" [{:?}]", std::env::var("abcd")); // Ok("This is the value")
    }

    pub fn set_var2(){
        print!("{}",
            if std::env::var("abcd").is_ok() {
                "Already defined" 
            } else {
                "Undefined"
            }
        );
        std::env::set_var("abcd", "This is the value"); 
        print!(", {}.", 
            match std::env::var("abcd") {
                Ok(value) => value,
                Err(err) => format!("Still undefined: {}", err),
            }
        );

    }
}
