#[doc(inline)]
#[path = "."]
/// # Module to handle USER ftp command 
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod user_handler{

    #[path = "common.rs"]
    mod common;

    #[path = "code.rs"]
    mod code;

    #[path = "messages.rs"]
    mod messages;

    use std::net::TcpStream;
    use common::common::*;
    use code::code::*;
    use messages::messages::*;

    /// # Strucutre to handle the USER command
    pub struct UserHandler {
        /// Username send by client
        pub username : Option<String>,
    }

    impl UserHandler{
        
        /// Call when USER command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* : stream use to send response to client
        /// 
        /// # Returns
        /// 
        /// * *true* :
        ///     - username exist and username is correcte
        /// 
        /// * *false* :
        ///     - username doesn't exist
        ///     - unsername exist and is not correct
        ///  
        pub fn execute(&self, stream: &mut TcpStream) -> bool {
            let usm = &self.username;
            let c : Code;
            let m : Message;
            let good_user : bool;

            // check if username exist
            match usm {
                Some(s) => {
                    let name : String = s.to_string();
                    if name == "anonymous"{
                        c = SPEC_PASSWORD_C;
                        m = SPEC_PASSWORD_M;
                        good_user = true;
                    } else {
                        c = SESSION_NO_OPEN_C;
                        m = ANO_ONLY_M;
                        good_user = false;
                    }

                },
                &None => {
                    c = UNVA_SYNTAX_ARGS_C;
                    m = UNVA_SYNTAX_ARGS_M;
                    good_user = false;
                }
            }

            write_line(format!("{} {}", c, m), stream);

            return good_user;
        }
        
    }


}