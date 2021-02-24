
/// # User object who communicate with user
///  
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
#[path = "."]
pub mod user{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "FtpHandler.rs"]
    mod ftp_handler;

    #[path = "common.rs"]
    mod common;

    use std::net::TcpStream;
    use std::io::{BufRead, BufReader, Write};
    use std::sync::mpsc;
    use messages::messages::*;
    use code::code::*;
    use ftp_handler::ftp_handler::FtpHandler;
    use std::time::Duration;
    use common::common::*;
    
    
    /// # Structure of an user of server
    pub struct User{

        /// Personnal TcpStream to send request to user
        pub server_stream : TcpStream,

        /// Message channel from main thread to user thread to say when server is shutdown
        pub stop : mpsc::Receiver<bool>,

        pub path : String,
    }

    impl User{

        
        /// Call by server tu handle (send and receive) users request in independant thread 
        /// 
        pub fn run(&mut self){    

            self.server_stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
            
            let mut request = String::new();
            let mut read_buffer = BufReader::new(self.server_stream.try_clone().unwrap());

            let mut handler : FtpHandler = FtpHandler::new(self.server_stream.try_clone().unwrap());


            write_line(format!("{} {}", WELCOM_C, WELCOM_M), &mut self.server_stream);
            
            while handler.running(){

                match self.stop.try_recv() {
                    Ok(true) => {
                        write_line(format!("{} {}", BYE_C, BYE_M), &mut self.server_stream);
                        break;
                    }
                    _ => {}
                }

                request.clear();
                
                match read_buffer.read_line(&mut request){
                    Ok(_) => {
                        
                        handler.request_handler(
                            request.lines().next().unwrap().split(" ").collect()
                        );

                        log::info!("recieve => {}", request);
                    },
                    Err(_) => {}
                }

                self.server_stream.flush().unwrap();

            }
        }

        
    }

}