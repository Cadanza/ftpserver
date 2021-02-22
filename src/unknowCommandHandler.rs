
#[path ="."]
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

    pub struct UnknowCommandHandler {} 

    impl UnknowCommandHandler {
        pub fn execute(&self, stream : &mut TcpStream){
            write_line(format!("{} {}", UNKNONW_COMMAND_C, UNKNOWN_COMMAND_M), stream);
        }
    }

}