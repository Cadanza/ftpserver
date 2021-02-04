# [warn(non_snake_case)]
use std::env;

fn main() {
    
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

}
