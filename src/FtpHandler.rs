

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

    #[path = "passifHandler.rs"]
    mod passiv_handler;

    #[path = "listHandler.rs"]
    mod list_handler;

    #[path  = "authHandler.rs"]
    mod auth_handler;

    use command::command::FtpCommand;
    use std::net::{TcpStream, TcpListener, Shutdown};
    use user_handler::user_handler::*;
    use unknow_command_handler::unknow_command_handler::*;
    use password_handler::password_handler::*;
    use quit_handler::quit_handler::*;
    use passiv_handler::passiv_handler::*;
    use list_handler::list_handler::*;
    use auth_handler::auth_handler::*;

    pub struct FtpHandler{
        running : bool,
        server_stream : TcpStream,
        passiv_port : Option<u16>,
        data_stream : Option<TcpStream>
    }
        

    impl FtpHandler{

        pub fn new(stream : TcpStream) -> FtpHandler {
            return FtpHandler{running : true, 
                server_stream : stream,
                passiv_port : None, 
                data_stream : None};
        }

        /// # Handle request send by user and call the good function to response at the rquest
        pub fn request_handler(&mut self, data : Vec<&str>) {
            

            let mut data_bis : Vec<String> = data.iter().map(|s | String::from(*s)).collect();


            let command : &str = data[0];
            let arg_pop : Option<String> = data_bis.pop();
            

            println!("{}", command);

            match command{

                "USER" => UserHandler{username : arg_pop}.execute(&mut self.server_stream),
                

                "AUTH" => AuthHandler{}.execute(&mut self.server_stream),
                
                "PASS" => PasswordHandler{password : arg_pop}.execute(&mut self.server_stream),

                "QUIT" => {
                    self.running = false;
                    QuitHandler{}.execute(&mut self.server_stream);
                },

                "LIST" => {
                    let dt : Option<TcpStream>;

                    match &self.data_stream{
                        Some(d) => dt = Some(d.try_clone().unwrap()),
                        None => dt = None,
                    }

                    ListHandler{data_stream : dt}.execute(&mut self.server_stream);
                    
                    match &self.data_stream{
                        Some(d) => d.shutdown(Shutdown::Both).expect("shutdown call failed"),
                        None => {}
                    }
                    

                    println!("ok3");
                }

                "PASV" => {
                    self.passiv_port = self.search_free_port();

                    PassivHandler{port : self.passiv_port}.execute(&mut self.server_stream);

                    match self.passiv_port {
                        Some(port) => {
                            let l = TcpListener::bind(("127.0.0.1", port)).unwrap();

                            for data_stream in l.incoming(){
                                match data_stream {
                                    Ok(stream) => {
                                        self.data_stream = Some(stream);
                                    },
                                    _ => {}
                                }
                                break;
                            }
                        },
                        None => {

                        }
                    }

                },

                _ => UnknowCommandHandler{}.execute(&mut self.server_stream),

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


        pub fn running(&mut self) -> bool{
            return self.running;
        }


        fn search_free_port(&mut self) -> Option<u16>{
            for port in 1025..65535{
                match TcpListener::bind(("127.0.0.1", port)){
                    Ok(l) => {
                        drop(l);
                        return Some(port);
                    },
                    _ => {}
                }
            }

            return None;
        }


        
    }
    
    
    
}