

#[path = "."]
pub mod ftp_handler{

    #[path = "command.rs"]
    mod command;

    #[path = "userHandler.rs"]
    mod user_handler;

    #[path = "unknowCommandHandler.rs"]
    mod unknow_command_handler;

    #[path = "passwordHandler.rs"]
    mod password_handler;

    #[path = "quitHandler.rs"]
    mod quit_handler;

    use command::command::FtpCommand;
    use std::net::{TcpStream, TcpListener};
    use user_handler::user_handler::*;
    use unknow_command_handler::unknow_command_handler::*;
    use password_handler::password_handler::*;
    use quit_handler::quit_handler::*;

    pub struct FtpHandler{
        running : bool,
        passiv_listener : Option<TcpListener>,
    }
    

    impl FtpCommand for UserHandler {
        fn execute(&self, stream : TcpStream){
            self.handler(stream);
        }
    }

    impl FtpCommand for UnknowCommandHandler {
        fn execute(&self, stream : TcpStream){
            self.handler(stream);
        }
    }

    impl FtpCommand for PasswordHandler {
        fn execute(&self, stream : TcpStream){
            self.handler(stream);
        }
    }

    impl FtpCommand for QuitHandler {
        fn execute(&self, stream : TcpStream){
            self.handler(stream);
        }
    }

    

    impl FtpHandler{

        pub fn new() -> FtpHandler {
            return FtpHandler{running : true, passiv_listener : None};
        }

        /// # Handle request send by user and call the good function to response at the rquest
        pub fn request_handler(&mut self, data : Vec<&str>) -> Box<dyn FtpCommand> {
                
            let mut data_bis : Vec<String> = data.iter().map(|s | String::from(*s)).collect();


            let command : &str = data[0];
            let arg_pop : Option<String> = data_bis.pop();
            

            println!("{}", command);

            match command{

                "USER" => return Box::new(UserHandler{username : arg_pop}),

                // "AUTH" => return (SESSION_NO_OPEN, AUTH_ERROR),
                
                "PASS" => return Box::new(PasswordHandler{password : arg_pop}),

                "QUIT" => {
                    self.running = false;
                    return Box::new(QuitHandler{});
                },

                // "LIST" => return self.list_handler(),

                // "PASV" => return self.passiv_handler(),

                _ => return Box::new(UnknowCommandHandler{}),

            }
        }


        // // fn list_handler(&mut self) -> (Code, &str){
            
        // //     for stream in self.data_listener.as_ref().unwrap().incoming(){

        // //         match stream {
        // //             Ok(mut stream) => {
        // //                 let o = Command::new("ls").arg("-n").output().expect("failed to execute process");
        // //                 stream.write_all(&o.stdout).unwrap();
        // //             }
        // //             Err(_) => {}
        // //         }
        // //     }

        // //     return (PASSIF_MODE_C, PASSIF_MODE_M);
        // // }

        // fn passiv_handler(&mut self) -> (Code, &str){

        //     let port : u16;
        //     let up1 : u16;
        //     let up2 : u16;

        //     let p = self.search_free_port();

        //     if p.is_none(){
        //         return (SERVICE_UNA_C, SERVICE_UNA_M);
        //     }

        //     self.data_listener = Some(p.unwrap());
        //     port = self.data_listener.as_ref().unwrap().local_addr().unwrap().port();
        //     up2 = port%256;
        //     up1 = (port-up2)/256;
            
        //     let m =format!("(127,0,0,1,{},{})", up1, up2);
            
        //     let ms = PASSIF_MODE_M;

        //     let rm : String = format!("{}{}", ms, m);


        //     return (PASSIF_MODE_C, Box::leak(rm.into_boxed_str()));
        // }


        pub fn running(&mut self) -> bool{
            return self.running;
        }

        // fn search_free_port(&mut self) -> Option<TcpListener>{
        //     for port in 1025..65535{
        //         match TcpListener::bind(("127.0.0.1", port)){
        //             Ok(l) => return Some(l),
        //             _ => {}
        //         }
        //     }

        //     return None;
        // }


        
    }
    
    
    
}