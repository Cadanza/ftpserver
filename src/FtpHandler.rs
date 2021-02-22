

#[path = "."]
pub mod ftp_handler{

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

    use std::net::{TcpStream, Shutdown};
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
        data_port : Option<u16>,
        data_stream : Option<TcpStream>,
        good_user : bool,
        good_psw : bool,
    }
        

    impl FtpHandler{

        pub fn new(stream : TcpStream) -> FtpHandler {
            return FtpHandler{running : true, 
                server_stream : stream,
                data_port : None, 
                data_stream : None,
                good_user : false,
                good_psw : false
            };
        }

        /// # Handle request send by user and call the good function to response at the rquest
        pub fn request_handler(&mut self, data : Vec<&str>) {
            

            let mut data_bis : Vec<String> = data.iter().map(|s | String::from(*s)).collect();


            let command : &str = data[0];
            let arg_pop : Option<String> = data_bis.pop();
            

            println!("{}", command);

            match command{

                "USER" => self.good_user = UserHandler{username : arg_pop}.execute(&mut self.server_stream),
                

                "AUTH" => AuthHandler{}.execute(&mut self.server_stream),
                
                "PASS" => {
                        self.good_psw = PasswordHandler{password : arg_pop, good_user : self.good_user}.execute(&mut self.server_stream);  
                },

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

                    ListHandler{data_stream : dt, session_open : self.session_open()}.execute(&mut self.server_stream);
                    
                    match &self.data_stream{
                        Some(d) => d.shutdown(Shutdown::Both).expect("shutdown call failed"),
                        None => {}
                    }
                }

                "PASV" => {

                    let passiv_ret = PassivHandler{session_open : self.session_open()}.execute(&mut self.server_stream);

                    self.data_port = passiv_ret.0;
                    self.data_stream = passiv_ret.1;
                    
                },

                _ => UnknowCommandHandler{}.execute(&mut self.server_stream),

            }
        }


        pub fn running(&mut self) -> bool{
            return self.running;
        }

        fn session_open(&self) -> bool {
            return self.good_psw && self.good_psw;
        }
        
    }
    
}