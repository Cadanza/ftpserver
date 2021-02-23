
#[path ="."]
/// # Module to handle all unknown command or non implemented ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod unknow_command_handler{

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

    /// # Structure to handle an unknow command
    pub struct UnknowCommandHandler {} 

    impl UnknowCommandHandler {

        /// Call when an unhnown command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// * **stream** *TcpStream* stream use to send response 
        /// 
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", UNKNONW_COMMAND_C, UNKNOWN_COMMAND_M), stream);
        }
    }

}