use std::env;
use std::io::{self, Write};

fn main() {
    // prompt the user for the environment variable name
    print!("Enter the environment variable name: ");
    // ensure the prompt is flushed to the console
    io::stdout().flush().unwrap();
    
    // read the input from stdin
    let mut env_var_name = String::new();
    io::stdin().read_line(&mut env_var_name).unwrap();
    
    // trim the newline from the input
    let env_var_name = env_var_name.trim();
    
    // attempt to retrieve the environment variable
    match env::var(env_var_name) {
        Ok(value) => print!("The value of '{}' is: {}", env_var_name, value),
        Err(_) => println!("Environment variable '{}' not set.", env_var_name),
    }
}
