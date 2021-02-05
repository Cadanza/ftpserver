mod user;

use log::LevelFilter;
use std::env;
use std::assert;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use user::user::User;

fn handle_client(mut stream : TcpStream){

    stream.write(b"220 hello\n").unwrap();
    stream.flush();

    thread::sleep_ms(5000);
}

fn main() {

    let _logger = simple_logging::log_to_file("test.log", LevelFilter::Info);

    log::info!("coucou");
    
    let args : Vec<String> = env::args().collect();
    
    assert!(args.len()>=3);

    //let f_root : &String = &args[1];
    let port : &String = &args[2];

    let addr : &str = &*format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(addr).unwrap();

        for stream in listener.incoming() {

            match stream{
                Ok(stream) => {
                    log::info!("new connection {}",stream.peer_addr().unwrap() );
                    //let mut u = User{ server_stream : stream};
                    thread::spawn(move || {
                        //u.run()
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("erreur {}", e);
                }
            }
            
        }
       // drop(listener);
    


}

