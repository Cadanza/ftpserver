#[path ="."]
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

    pub struct AuthHandler {} 

    impl AuthHandler {
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", SESSION_NO_OPEN, AUTH_ERROR), stream);
        }
    }

}