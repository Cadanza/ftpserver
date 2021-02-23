#[path ="."]

/// # Module to handle AUTH command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod auth_handler{

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

    /// # Structure to handler the AUTH command
    pub struct AuthHandler {} 

    impl AuthHandler {

        /// Call when AUTH is recieve by ftp handler
        /// 
        /// Send response to the client he must have to use USER and PASS command to log in server 
        /// 
        /// # Argument
        /// 
        /// - **stream** *TcpStream* : stream use to send response to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", SESSION_NO_OPEN_C, AUTH_ERROR_M), stream);
        }
    }

}