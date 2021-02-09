#[path = "./FTPComm/messages.rs"]
mod messages;

#[path = "./FTPComm/code.rs"]
mod code;


pub mod user{
    use std::net::TcpStream;
    use std::io::{BufRead, BufReader, Write};
    use super::messages::messages;
    use super::code::code;
    

    pub struct User{
        pub server_stream : TcpStream,
    }

    impl User{

        pub fn run(&mut self){    
            
            let mut request = String::new();
            let mut read_buffer = BufReader::new(self.server_stream.try_clone().unwrap());
            let data : Vec<&str>;

            self.connect();
            loop{
                read_buffer.read_line(&mut request).unwrap();

                log::info!("recieve > {}", request);

                data = request.split(" ").collect();

                

                self.server_stream.flush().unwrap();

                if 

            }
        }

        fn connect(&mut self){
            self.send_request(code::WELCOME, messages::WELCOM);
        }


        fn send_request(&mut self, code : &str, msg : &str){
            let req : &str =  &*format!("{} {}\n", code, msg);

            let log_req : String = req.into();

            let tcp_req : &[u8] = req.as_bytes();

            log::info!("send => {}", log_req);

            self.server_stream.write(tcp_req).unwrap();
                
            
        }

        
    }

}