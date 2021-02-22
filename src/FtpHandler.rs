

#[path = "."]
pub mod ftp_handler{

    #[path = "common.rs"]
    mod common;

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

    #[path = "sessionNoOpen.rs"]
    mod session_no_open_handler;

    use common::common::*;
    use std::net::{TcpStream, TcpListener, Shutdown};
    use user_handler::user_handler::*;
    use unknow_command_handler::unknow_command_handler::*;
    use password_handler::password_handler::*;
    use quit_handler::quit_handler::*;
    use passiv_handler::passiv_handler::*;
    use list_handler::list_handler::*;
    use auth_handler::auth_handler::*;
    use session_no_open_handler::session_no_open_handler::*;

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
                    if self.good_user{
                        self.good_psw = PasswordHandler{password : arg_pop}.execute(&mut self.server_stream);
                    } else {
                        SessionNoOpenHandler{}.execute(&mut self.server_stream);
                    }
                        
                },

                "QUIT" => {
                    self.running = false;
                    QuitHandler{}.execute(&mut self.server_stream);
                },

                "LIST" => {

                    if !self.session_open(){
                        SessionNoOpenHandler{}.execute(&mut self.server_stream);
                        return;
                    }

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
                }

                "PASV" => {

                    if !self.session_open(){
                        SessionNoOpenHandler{}.execute(&mut self.server_stream);
                        return;
                    }

                    self.data_port = search_free_port();

                    PassivHandler{port : self.data_port}.execute(&mut self.server_stream);

                    match self.data_port {
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


        pub fn running(&mut self) -> bool{
            return self.running;
        }




        fn session_open(&self) -> bool {
            return self.good_psw && self.good_psw;
        }
        
    }
    
}