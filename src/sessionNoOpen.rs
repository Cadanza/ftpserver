
#[path ="."]
pub mod session_no_open_handler{

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

    pub struct SessionNoOpenHandler {} 

    impl SessionNoOpenHandler {
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", SESSION_NO_OPEN, SESSION_NO_OPEN_MES), stream);
        }
    }

}