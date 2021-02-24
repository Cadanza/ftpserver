#[path ="."]
/// Module to handle QUIT ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod port_handler{

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

    /// # Structure to handle PORT command
    pub struct PortHandler {
        pub data : String
    } 

    impl PortHandler {

        /// Call when QUIT command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", BYE_C, BYE_M), stream);
        }
    }

}