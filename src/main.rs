use std::env;
use std::io::{self, Write};

fn main() {
    // prompt the user for the environment variable name
    print!("Enter the name of the environment variable: ");
    // ensure the prompt is flushed to the console
    if let Err(e) = io::stdout().flush() {
        eprintln!("Failed to flush stdout: {}", e);
        return;
    }
    
    // read the input from stdin
    let mut env_var_name = String::new();
    if let Err(e) = io::stdin().read_line(&mut env_var_name) {
        eprintln!("Failed to read line: {}", e);
        return;
    }
    
    // trim the newline from the input
    let env_var_name = env_var_name.trim();
    
    // attempt to retrieve the environment variable
    match env::var(env_var_name) {
        Ok(value) => println!("The value of the environment variable '{}' is: {}", env_var_name, value),
        Err(_) => println!("The environment variable '{}' was not found", env_var_name),
    }
}
