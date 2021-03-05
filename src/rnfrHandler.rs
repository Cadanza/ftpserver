#[path ="."]
/// # Module to handle RNFR ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod rnfr_handler{

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

    /// # Structure to handle RNFR command
    pub struct RnfrHandler {

        /// say if user session is open
        pub session_open : bool,

        /// path of directory to rename
        /// - *String* : a directory was found on argument command
        /// - *None* : no argument was passed with command
        pub path : Option<String>,

        /// actual path of user on server file system
        pub actual_path : String,

        /// server root directory
        pub root : String
    } 

    impl RnfrHandler {

        /// Call when RNFR command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        /// # Returns
        /// 
        /// - *Option<String> :
        ///     - *String* : The directory was found and it is ready to be rename
        ///     - *None* : 
        ///         - Session is not open 
        ///         - No path was given on argument
        ///         - Directory was not found
        ///  
        pub fn execute(&self, stream : &mut TcpStream) -> Option<String> {

            let c : Code;
            let m : Message;

            let ret : Option<String>;

            if !self.session_open {
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
                ret = None;
            } else {

                match &self.path {
                    Some(dir) => {
                        match relative_to_absolute_path(
                            format!("{}", self.root),
                            format!("{}", self.actual_path),
                            format!("{}", dir)) {
                            Some(p) => {
                                if directory_exist(format!("{}", p)){

                                    c = FILE_SERV_WAIT_C;
                                    m = FILE_SERV_WAIT_M;
                                    ret = Some(format!("{}", p));

                                } else {

                                    c = FILE_NOT_ACCESS_C;
                                    m = FILE_NOT_ACCESS_M;
                                    ret = None;
                                    
                                }
                                
                            },
                            None => {
                                c = FILE_NOT_ACCESS_C;
                                m = FILE_NOT_ACCESS_M;
                                ret = None;
                            }
                        }
                    },
                    None => {
                        c = UNVA_SYNTAX_ARGS_C;
                        m = UNVA_SYNTAX_ARGS_M;
                        ret = None;
                    }
                }

            }

            write_line(format!("{} {}", c, m), stream);

            return ret;
        }
    
    }

    

}