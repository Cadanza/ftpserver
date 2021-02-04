
use log::LevelFilter;
use std::env;
use std::assert;

fn main() {

    let logger = simple_logging::log_to_file("test.log", LevelFilter::Info);

    log::info!("coucou");
    
    let args : Vec<String> = env::args().collect();
    
    assert!(args.len()>=3);

    let froot : &String = &args[1];
    let port : &String = &args[2];

    println!("{} -- {}", froot, port);

}
