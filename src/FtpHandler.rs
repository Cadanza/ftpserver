

#[path = "."]
pub mod ftp_handler{

    #[path = "command.rs"]
    mod command;

    #[path = "code.rs"]
    mod code;

    #[path = "messages.rs"]
    mod messages;

    #[path = "userHandler.rs"]
    mod userHandler;



    use code::code::*;
    use messages::messages::*;
    use command::command::FtpCommand;
    use std::net::TcpStream;
    use userHandler::userHandler::*;

    pub struct FtpHandler{
        running : bool,
    }

    pub struct Nada{
    }

    impl FtpCommand for Nada{
        fn execute(&self, stream : TcpStream){
            //write_line(format!("{}{}", UNKNONW_COMMAND_C, UNKNOWN_COMMAND_MES), stream);
        }
    }

    impl FtpCommand for UserHandler {
        fn execute(&self, stream : TcpStream){
            self.user_handler_exe(stream);
        }
    }

    

    impl FtpHandler{

            pub fn new() -> FtpHandler {
        return FtpHandler{running : true};
    }

        /// # Handle request send by user and call the good function to response at the rquest
        pub fn request_handler(&mut self, data : Vec<&str>) -> Box<dyn FtpCommand> {
                
            let mut data_bis : Vec<String> = data.iter().map(|s | String::from(*s)).collect();

            //let arg_pop : Option<&str> = data.pop();
            let command : &str = data[0];
            let arg_pop : Option<String> = data_bis.pop();
            

            println!("{}", command);

            match command{

                "USER" => return Box::new(UserHandler{username : arg_pop}),

                // "AUTH" => return (SESSION_NO_OPEN, AUTH_ERROR),
                
                // "PASS" => return self.pass_handler(arg_pop),

                // "QUIT" => return self.quit_handler(),

                // "LIST" => return self.list_handler(),

                // "PASV" => return self.passiv_handler(),

                // _ => return self.no_found_handler(),

                _ => return Box::new(Nada{}),

            }
        }





        // fn pass_handler(&mut self, psw : Option<&str>) -> (Code, &str){

        //     if psw.is_none(){
        //         return (SYNTAX_ARGS_ERROR, UNVA_SYNTAX_ARGS);
        //     }

        //     let password : &str = psw.unwrap();
        //     let c : Code;
        //     let m : &str;

        //     if password == "anonymous" {
        //         c = SESSION_OPEN;
        //         m = SESSION_OPEN_MES;
        //     } else {
        //         c = SESSION_NO_OPEN;
        //         m = ANO_ONLY;
        //     }

        //     return (c, m);
        // }

        // fn no_found_handler(&mut self) -> (Code, &str) {
        //     return (UNKNONW_COMMAND_C, UNKNOWN_COMMAND_MES);
        // }

        // fn quit_handler(&mut self) -> (Code, &str){
        //     self.running = false;
        //     return (BYE, BYE_MES);
        // }

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