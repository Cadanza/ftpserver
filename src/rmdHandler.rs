#[path ="."]
/// # Module to handle RMD ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod rmd_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "common.rs"]
    mod common;

    #[path = "fileSystemHandler.rs"]
    mod file_system_handler;

    use std::net::TcpStream;
    use messages::messages::*;
    use code::code::*;
    use common::common::*;
    use file_system_handler::file_system_handler::*;

    /// # Structure to handle RMD command
    pub struct RmdHandler {

        /// says if user session if open
        pub session_open : bool,

        /// path of folder to remove
        /// - *String* : path was found on argument command
        /// - *None* : no argument given on command argument
        pub path : Option<String>,

        /// actual path of user on server file system
        pub actual_path : String,

        /// server root directory
        pub root : String
    } 

    impl RmdHandler {

        /// Call when RMD command is recieve by ftp handler
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
                    Some(dir) => {
                        match relative_to_absolute_path(
                            format!("{}", self.root),
                            format!("{}", self.actual_path),
                            format!("{}", dir)) {

                            Some(abs_dir) => {
                                if remove_folder(abs_dir){
                                    c = FILE_SERVICE_FINISH_C;
                                    m = FILE_SERVICE_FINISH_M;
                                } else {
                                    c = FILE_NOT_ACCESS_C;
                                    m = FILE_NOT_ACCESS_M;
                                }
                            },
                            None => {
                                c = FILE_NOT_ACCESS_C;
                                m = FILE_NOT_ACCESS_M;
                            }
                        }
                    },
                    None => {
                        c = UNVA_SYNTAX_ARGS_C;
                        m = UNVA_SYNTAX_ARGS_M;
                    }
                }

            }

            write_line(format!("{} {}", c, m), stream);
        }
    
    }
}