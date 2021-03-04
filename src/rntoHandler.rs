#[path ="."]
/// Module to handle RNTO ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod rnto_handler{

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
    use std::fs;

    /// # Structure to handle MDK command
    pub struct RntoHandler {
        pub session_open : bool,
        pub path : Option<String>,
        pub name : Option<String>,
    } 

    impl RntoHandler {

        /// Call when RNTO command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream){

            let c : Code;
            let m : Message;

            if !self.session_open {
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
            } else {
                match &self.path {
                    Some(p) => {
                        match &self.name {
                            Some(n) => {
                                let name =self.absolute_name(format!("{}", p), format!("{}", n));

                                match fs::rename(format!("{}", p), name){
                                    Ok(_) => {
                                        c = FILE_SERVICE_FINISH_C;
                                        m = FILE_SERVICE_FINISH_M;
                                    },
                                    Err(_) => {
                                        c = UNVA_FILE_NAME_C;
                                        m = UNVA_FILE_NAME_M;
                                    }
                                }
                            },
                            None => {
                                c = UNVA_SYNTAX_ARGS_C;
                                m = UNVA_SYNTAX_ARGS_M;
                            }
                        }
                    },
                    None => {
                        c = BAD_COM_SEQ_C;
                        m = BAD_COM_SEQ_M;
                    }
                }
            }

            write_line(format!("{} {}", c, m), stream);
        }


        
        /// Convert a simple name to new absolute path of rename file
        /// 
        /// # Arguments
        /// 
        /// - **last_name** *String* : absolute path of last name
        /// - **new_name** *String* : new name of file/folder
        /// 
        /// # Returns
        /// 
        /// - *String* : new absolute path of file/folder with it new name
        /// 
        fn absolute_name(&self, last_name : String, new_name : String) -> String {
            let mut cut_ln : Vec<&str> = last_name.split("/").collect();
            let cut_name : &str = &*new_name;

            cut_ln.pop();
            cut_ln.push(cut_name);

            let ret : String = cut_ln.join("/");

            return format!("{}", ret);
        }
    
    }

    

}