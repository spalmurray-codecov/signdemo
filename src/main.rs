use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = &args[1];

    let result: bool = signdemo::sign_file(file);
    
    if !result {
        exit(1);
    }

    println!("Signed the file.\nOutput is: {}.asc", file);
    exit(0);
}

