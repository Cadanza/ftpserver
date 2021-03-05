#[path ="."]
/// # Module to handle RNTO ftp command
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

    #[path = "fileSystemHandler.rs"]
    mod file_system_handler;

    use std::net::TcpStream;
    use messages::messages::*;
    use code::code::*;
    use common::common::*;
    use file_system_handler::file_system_handler::*;

    /// # Structure to handle RNTO command
    pub struct RntoHandler {

        /// say if user session is open or not
        pub session_open : bool,

        /// Path of directory to rename. Fiound by RNFR handler
        /// - *String* : A directory was found
        /// - *None* : No directory was found
        pub path : Option<String>,

        /// New name of directory
        /// - *String* : Name was found on command argument
        /// - *None* : no argument was found on command
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
                                let name =name_to_absolute_path(format!("{}", p), format!("{}", n));

                                if rename_dir(format!("{}", p), name){
                                    
                                    c = FILE_SERVICE_FINISH_C;
                                    m = FILE_SERVICE_FINISH_M;

                                }else {

                                    c = UNVA_FILE_NAME_C;
                                    m = UNVA_FILE_NAME_M;
                                    
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
    }

}