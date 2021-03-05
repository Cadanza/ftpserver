
#[path ="."]
/// # Module to handle PASS ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod password_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "common.rs"]
    mod common;

    use std::net::TcpStream;
    use messages::messages::*;
    use code::code::*;
    use common::common::*;

    /// # Structure to handle the PASS command
    /// 
    /// # Parameters:
    /// 
    /// * `password` *Option<String>* : password send by client (could be None if no password send)
    /// * `good_user` *bool* : true if username was correct, false else.
    ///  
    pub struct PasswordHandler {

        /// password recieve as argument of PASS command
        /// - *String* if argument was found
        /// - *None* else
        pub password : Option<String>,

        /// true if username was correct
        pub good_user : bool,
    } 

    impl PasswordHandler {
         
        /// Call when PASS command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// * **stream** *TcpStream* : Stream use to send response data to client
        /// 
        /// # Returns
        /// 
        /// - *true* :
        ///     - if password is correct
        /// 
        /// - *false* : 
        ///     - if password is not correct or no password was found by handler
        ///  
        pub fn execute(&self, stream : &mut TcpStream) -> bool{

            let c : Code;
            let m : Message;
            let pasw = &self.password;
            let good_psw : bool;

            if !self.good_user{
                log::info!("No or false user name was recieved");
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
                good_psw = false;
            } else {

                match pasw {
                    Some(p) => {
    
                        if p == "anonymous" {
                            log::info!("Password is correct");
                            c = SESSION_OPEN_C;
                            m = SESSION_OPEN_M;
                            good_psw = true;
                        } else {
                            log::info!("Password is not correct");
                            c = SESSION_NO_OPEN_C;
                            m = ANO_ONLY_M;
                            good_psw = false;
                        }
                    },
                    &None => {
                        log::info!("No password found with ftp command");
                        c = UNVA_SYNTAX_ARGS_C;
                        m = UNVA_SYNTAX_ARGS_M;
                        good_psw = false;
                    }
                }

            }

            write_line(format!("{} {}", c, m), stream);

            return good_psw;
        }
    }

}