
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
    } 

    impl ListHandler {
        pub fn execute(&self, stream : &mut TcpStream){

            match &self.data_stream {
                Some(dts) => {

                    let mut s = dts.try_clone().unwrap();

                    write_line(format!("{} {}", DATA_COME_C, DATA_COME_M), stream);
                    let output = Command::new("ls").arg("-n").output().expect("failder to execute process");
                    
                    let o = String::from_utf8_lossy(&output.stdout);
                    write_data(format!("{}", o), &mut s);

                    write_line(format!("{} {}", DATA_SEND_C, DATA_SEND_M), stream);

                },
                None => {}
            }

            
            
        }

    }

}