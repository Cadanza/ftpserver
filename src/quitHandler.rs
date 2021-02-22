#[path ="."]
pub mod quit_handler{

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

    pub struct QuitHandler {} 

    impl QuitHandler {
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", BYE, BYE_MES), stream);
        }
    }

}