#[path ="."]

/// # Module to handle CWD and CDUP ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod cd_handler{

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

    /// # Structure to handle CWD command
    pub struct CdHandler {

        /// true if session is correctly open
        pub session_open : bool,

        /// actual path location of current user
        pub actual_path : String,

        /// directory where user want to go
        /// It can be *None* if no argument was founded after CWD keyword
        pub directory : Option<String>,

        /// server root directory
        pub root : String
    } 

    impl CdHandler {

        /// Call when CWD or CDUP command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* stream use to send response to user
        /// 
        /// # Returns
        /// 
        /// - *String* : new actual path of user
        /// 
        pub fn execute(&self, stream : &mut TcpStream) -> String {

            let ret : String;

            let c : Code;
            let m : Message;

            match &self.directory {
                Some(dir) => {

                    match relative_to_absolute_path(
                        format!("{}", self.root), 
                        format!("{}", self.actual_path), 
                        dir.to_string()){

                        Some(d) => {

                            if directory_exist(format!("{}", d)) {

                                log::info!("move path user to {}", d);
                                c = FILE_SERVICE_FINISH_C;
                                m = FILE_SERVICE_FINISH_M;
                                ret = format!("{}/", d);

                            } else {

                                log::info!("{} doesn't exist", dir);
                                c = FILE_NOT_ACCESS_C;
                                m = FILE_NOT_ACCESS_M;
                                ret = format!("{}", self.actual_path);
                            }
                        },
                        
                        None => {

                            log::info!("{} is above root folder", dir);
                            c = FILE_NOT_ACCESS_C;
                            m = FILE_NOT_ACCESS_M;
                            ret = format!("{}", self.actual_path);

                        }
                    }

                },
                None => {

                    log::info!("No arguments was found with ftp command");
                    c = UNVA_SYNTAX_ARGS_C;
                    m = UNVA_SYNTAX_ARGS_M;
                    ret = format!("{}", self.actual_path);

                }
            }

            write_line(format!("{} {}", c, m), stream);


            return ret;
        }

    }

}