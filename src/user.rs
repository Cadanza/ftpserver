#[path = "./messages.rs"]
mod messages;

#[path = "./code.rs"]
mod code;

#[path = "./FtpHandler.rs"]
mod FtpHandler;


/// # User object who communicate with user
pub mod user{
    use std::net::TcpStream;
    use std::io::{BufRead, BufReader, Write};
    use std::io::prelude::*;
    use super::messages::messages::*;
    use super::code::code::*;
    use super::FtpHandler::FtpHandler;
    
    
    /// # Structure of an user of server
    /// 
    /// Contains TcpStream to send request to user
    pub struct User{
        pub server_stream : TcpStream,
    }

    impl User{

        ///
        /// call by server tu handle (send and receive) users request in independant thread 
        /// 
        pub fn run(&mut self){    
            
            let request = String::new();
            let mut read_buffer = BufReader::new(self.server_stream.try_clone().unwrap());
            let mut data : Vec<&str>;

            self.connect();
            loop{
                read_buffer.read_line(&mut request).unwrap();

                log::info!("recieve > {}", request);

                data = request.split(" ").collect();

                self.send_request(FtpHandler::request_handler(data));

                self.server_stream.flush().unwrap();


            }
        }

        /// 
        /// # send connection message to user
        /// 
        fn connect(&mut self){
            self.send_request((WELCOME, WELCOM_MES));
        }

        /// # send requestion to user
        /// 
        /// ## arguments
        /// * self
        /// * code : &str => code of response
        /// * msg : &str => message send to user
        /// 
        /// **request will be write on log file**
        /// 
        fn send_request(&mut self, ret : (Code, Message)){
            let req : &str =  &*format!("{} {}\n", ret.0, ret.1);

            let log_req : String = req.into();

            let tcp_req : &[u8] = req.as_bytes();

            log::info!("send => {}", log_req);

            self.server_stream.write(tcp_req).unwrap();
                
            
        }

        
    }

}