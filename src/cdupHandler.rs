#[path ="."]
/// Module to handle CDUP ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod cdup_handler{

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

    /// # Structure to handle CDUP command
    pub struct CdupHandler {

        /// true if session is correctly open
        pub session_open : bool,

        /// actual path location of current user
        pub actual_path : String,

        /// root directory, can't go up
        pub root : String
    } 

    impl CdupHandler {

        /// Call when CDUP command is recieve by ftp handler
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

            if !self.session_open {

                ret = format!("{}", self.actual_path);
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;

            } else {
                let mut splited_actual_path : Vec<&str> = self.actual_path.split("/").collect(); 

                let mut splited_root_path : Vec<&str> = self.root.split("/").collect();

                splited_actual_path.pop(); //remove ""
                splited_root_path.pop(); //remove ""

                splited_actual_path.pop(); // remove directory

                if splited_actual_path.len() >= splited_root_path.len(){
                    
                    ret = format!("{}/", splited_actual_path.join("/"));
                    c = CONCLUD_COMMAND_C;
                    m = CONCLUD_COMMAND_M;

                } else {

                    c = FILE_NOT_ACCESS_C;
                    m = FILE_NOT_ACCESS_M;
                    ret = format!("{}",self.root);
                }
            }

            write_line(format!("{} {}", c, m), stream);

            return ret;
        }

    }

}