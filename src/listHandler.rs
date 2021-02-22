
#[path ="."]
pub mod list_handler{

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
    use std::process::Command;

    pub struct ListHandler {
        pub data_stream : Option<TcpStream>,
        pub session_open : bool,
    } 

    impl ListHandler {
        pub fn execute(&self, stream : &mut TcpStream){

            if self.session_open {
                write_line(format!("{} {}", SESSION_NO_OPEN, SESSION_NO_OPEN_MES), stream);
            } else {
                self.send_data(stream);
            }

        }

        fn send_data(&self, stream : &mut TcpStream) {

            match &self.data_stream {
                Some(dts) => {

                    let mut s = dts.try_clone().unwrap();

                    write_line(format!("{} {}", DATA_COME_C, DATA_COME_M), stream);
                    
                    write_data(self.get_command_res(), &mut s);

                    write_line(format!("{} {}", DATA_SEND_C, DATA_SEND_M), stream);

                },
                None => {}
            }
        }

        fn get_command_res(&self) -> String {
            let output = Command::new("ls").arg("-n").output().expect("failder to execute process");
                    
            let o = String::from_utf8_lossy(&output.stdout);

            return format!("{}", o);
        }

    }

}