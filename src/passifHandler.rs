
#[path ="."]
pub mod passiv_handler{

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

    pub struct PassivHandler {
        pub port : Option<u16>,
    } 

    impl PassivHandler {
        pub fn execute(&self, stream : &mut TcpStream){
            let up1 : u16;
            let up2 : u16;

            let c : Code;
            let m :  &str;
            let end : String;

            match self.port {
                Some(p) => {
                    up2 = p%256;
                    up1 = (p-up2)/256;

                    c = PASSIF_MODE_C;
                    m = PASSIF_MODE_M;

                    end = format!(" (127,0,0,1,{},{})", up1, up2);
                }
                None => {
                    c = SERVICE_UNA_C;
                    m = SERVICE_UNA_M;
                    end = String::from("");
                }
            }

           write_line(format!("{} {}{}", c, m, end), stream);
        }

    }

}