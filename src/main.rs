mod user;
use log::LevelFilter;
use std::env;
use std::assert;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use user::user::User;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;



fn main() {

    let _logger = simple_logging::log_to_file("test.log", LevelFilter::Info);

    let running = Arc::new(AtomicBool::new(false));

    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("Ertror setting Ctrl-C handler");
    
    let args : Vec<String> = env::args().collect();
    
    assert!(args.len()>=3);

    //let f_root : &String = &args[1];
    let port : &String = &args[2];

    let addr : &str = &*format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(addr).unwrap();

    listener.set_nonblocking(true).unwrap();



    for stream in listener.incoming() {



        match stream{
            Ok(stream) => {
                log::info!("new connection {}",stream.peer_addr().unwrap() );
                let mut u = User{ server_stream : stream};
                thread::spawn(move || {
                    u.run()
                });
            }
            Err(_) => {
            }
        }

        if running.load(Ordering::SeqCst){
            break;
        }
        
    }
    

    println!("bye");
    drop(listener);
    


}

