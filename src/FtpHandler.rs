

#[path = "."]
/// # FTH Command Handler
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
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

    #[path = "portHandler.rs"]
    mod port_handler;


    use std::net::{TcpStream, Shutdown};
    use user_handler::user_handler::*;
    use unknow_command_handler::unknow_command_handler::*;
    use password_handler::password_handler::*;
    use quit_handler::quit_handler::*;
    use passiv_handler::passiv_handler::*;
    use list_handler::list_handler::*;
    use auth_handler::auth_handler::*;
    use port_handler::port_handler::*;

    /// # Structure who contains variables to handle ftp
    pub struct FtpHandler{

        /// - *true* : default value
        /// - *false* : set to false when QUIT command is recieve
        running : bool,

        /// Stream use to communicatee with user
        server_stream : TcpStream,

        /// Port use to open TcpListener and send data
        /// - *u16* : port use to send data
        /// - *None* : If no free port was found to send data
        data_port : Option<u16>,

        /// Stream use to send data
        /// - *TcpStream* : a free port was found and client connect himself to data listener
        /// - *None* : no free port was found
        data_stream : Option<TcpStream>,

        /// Good username was sent by user
        good_user : bool,

        /// Good password was sent by user
        good_psw : bool,
    }
        

    impl FtpHandler{

        /// Create a new FtpHandler strucutre initialize with user stream
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* user stream
        /// 
        /// # Returns
        /// 
        /// - FtpHandler struct with :
        ///     - running = true
        ///     - server_stream = stream
        ///     - data_port = None
        ///     - data_stream = None
        ///     - good_user = false
        ///     - good_psw = false
        /// 
        pub fn new(stream : TcpStream) -> FtpHandler {
            return FtpHandler{running : true, 
                server_stream : stream,
                data_port : None, 
                data_stream : None,
                good_user : false,
                good_psw : false
            };
        }

        /// # Handle request send by user and call the good function to response at the request
        /// 
        /// # Arguments
        /// 
        /// * **data** *Vec<&str>* vector contains command and command arguments
        /// 
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
                        None => {
                            println!("nop");
                            dt = None
                        }
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

                "PORT" => {
                    let activ_ret = PortHandler{data : arg_pop, session_open : self.session_open()}.execute(&mut self.server_stream);

                    self.data_port = activ_ret.0;
                    self.data_stream = activ_ret.1;

                }

                _ => UnknowCommandHandler{}.execute(&mut self.server_stream),

            }
        }

        /// Return running parameter
        /// 
        /// Running parameter is set to false when QUIT command is handle
        /// 
        pub fn running(&mut self) -> bool{
            return self.running;
        }

        /// Return true if the good username and the good password
        fn session_open(&self) -> bool {
            return self.good_psw && self.good_psw;
        }
        
    }
    
}