
/// # User object who communicate with user
#[path = "."]
pub mod user{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "FtpHandler.rs"]
    mod ftp_handler;

    #[path = "command.rs"]
    mod command;


    use std::net::TcpStream;
    use std::io::{BufRead, BufReader, Write};
    use std::sync::mpsc;
    use messages::messages::*;
    use code::code::*;
    use ftp_handler::ftp_handler::FtpHandler;
    use std::time::Duration;
    
    
    /// # Structure of an user of server
    /// 
    /// Contains TcpStream to send request to user
    pub struct User{
        pub server_stream : TcpStream,
        pub stop : mpsc::Receiver<bool>,
        pub path : String,
    }

    impl User{

        ///
        /// call by server tu handle (send and receive) users request in independant thread 
        /// 
        pub fn run(&mut self){    

            self.server_stream.set_read_timeout(Some(Duration::new(1, 0))).unwrap();
            
            let mut request = String::new();
            let mut read_buffer = BufReader::new(self.server_stream.try_clone().unwrap());

            let mut handler : FtpHandler = FtpHandler::new(self.server_stream.try_clone().unwrap());


            self.connect();
            
            while handler.running(){

                match self.stop.try_recv() {
                    Ok(true) => {
                        self.send_request((BYE, BYE_MES));
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
        fn send_request(&mut self, ret : (Code, &str)){
            let req : &str =  &*format!("{} {}\n", ret.0, ret.1);

            let log_req : String = req.into();

            let tcp_req : &[u8] = req.as_bytes();

            log::info!("send => {}", log_req);

            self.server_stream.write(tcp_req).unwrap();
                
            
        }

        
    }

}