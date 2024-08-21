use std::env;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 
        || args[1] == "help"
        || args[1] == "-h" {
        print_help();
        return;
    }
    
    let env_var_name = &args[1];
        
    match env::var(env_var_name) {
        Ok(value) => println!("The value of the environment variable '{}' is: {}", env_var_name, value),
        Err(_) => println!("The environment variable '{}' was not found", env_var_name),
    }
}

fn print_help() {
    println!("Usage: envar <ENV_VAR>");
    println!("Retrieves the value of the specified environment variable.");
    println!();
    println!("Options:");
    println!(" -h, --help       Show this help message");
    println!();
    println!("Examples:");
    println!(" envar ENV_VAR    Retrieves the value of the ENV_VAR environment variable");
}