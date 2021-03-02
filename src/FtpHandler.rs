

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

    #[path = "cwdHandler.rs"]
    mod cwd_handler;

    #[path = "cdupHandler.rs"]
    mod cdup_handler;


    use std::net::{TcpStream, Shutdown};
    use user_handler::user_handler::*;
    use unknow_command_handler::unknow_command_handler::*;
    use password_handler::password_handler::*;
    use quit_handler::quit_handler::*;
    use passiv_handler::passiv_handler::*;
    use list_handler::list_handler::*;
    use auth_handler::auth_handler::*;
    use port_handler::port_handler::*;
    use cwd_handler::cwd_handler::*;
    use cdup_handler::cdup_handler::*;

    /// # Structure who contains variables to handle ftp
    pub struct FtpHandler{

        /// - *true* : default value
        /// - *false* : set to false when QUIT command is recieve
        running : bool,

        /// Stream use to communicatee with user
        server_stream : TcpStream,

        /// Stream use to send data
        /// - *TcpStream* : a free port was found and client connect himself to data listener
        /// - *None* : no free port was found
        data_stream : Option<TcpStream>,

        /// Good username was sent by user
        good_user : bool,

        /// Good password was sent by user
        good_psw : bool,

        actual_path : String,

        root_path : String,
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
        pub fn new(stream : TcpStream, root : String) -> FtpHandler {
            return FtpHandler{running : true, 
                server_stream : stream,
                data_stream : None,
                good_user : false,
                good_psw : false,
                actual_path : format!("{}", root),
                root_path : root,
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

                    
                    ListHandler{data_stream : dt, session_open : self.session_open(), path : self.get_path()}.execute(&mut self.server_stream);
                    
                    match &self.data_stream{
                        Some(d) => d.shutdown(Shutdown::Both).expect("shutdown call failed"),
                        None => {}
                    }
                }

                "PASV" => {

                    let passiv_ret = PassivHandler{session_open : self.session_open()}.execute(&mut self.server_stream);

                    self.data_stream = passiv_ret;

                    
                },

                "PORT" => {
                    let activ_ret = PortHandler{data : arg_pop, session_open : self.session_open()}.execute(&mut self.server_stream);

                    self.data_stream = activ_ret;

                },

                "CWD" => {
                    self.actual_path = CwdHandler{
                        session_open : self.session_open(), 
                        actual_path : self.get_path(), 
                        directory : arg_pop,
                        root : self.get_root()
                    }.execute(&mut self.server_stream);

                },

                "CDUP" => {
                    self.actual_path = CdupHandler{
                        session_open : self.session_open(),
                        actual_path : self.get_path(),
                        root : self.get_root(),
                    }.execute(&mut self.server_stream);
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

        fn get_path(&self) -> String {
            return format!("{}", self.actual_path);
        }

        fn get_root(&self) -> String {
            return format!("{}", self.root_path);
        }
        
    }
    
}