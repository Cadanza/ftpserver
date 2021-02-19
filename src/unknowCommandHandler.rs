
#[path ="."]
pub mod unknow_command_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "command.rs"]
    mod command;

    use std::net::TcpStream;
    use messages::messages::*;
    use code::code::*;
    use command::command::*;

    pub struct UnknowCommandHandler {} 

    impl UnknowCommandHandler {
        pub fn handler(&self, stream : TcpStream){
            write_line(format!("{} {}", UNKNONW_COMMAND_C, UNKNOWN_COMMAND_MES), stream);
        }
    }

}