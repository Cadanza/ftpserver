
#[path ="."]
pub mod list_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "command.rs"]
    mod command;


    use std::net::{TcpStream, TcpListener};
    use messages::messages::*;
    use code::code::*;
    use command::command::*;
    use std::process::Command;

    pub struct ListHandler {
        pub port : Option<u16>,
    } 

    impl ListHandler {
        pub fn execute(&self, stream : TcpStream){

            // match self.port {
            //     Some(port) => {

            //         let listener : TcpListener = TcpListener::bind(("127.0.0.1", port)).unwrap();

            //         for s in listener.incoming(){

            //             match s {
            //                 Ok(data_stream) => {
            //                     let mut s1 = stream.try_clone().unwrap();

            //                     write_line(format!("{} {}", DATA_COME_C, DATA_COME_M), stream);
            //                     let output = Command::new("ls").arg("-n").output().expect("failder to execute process");
                                
            //                     let o = String::from_utf8_lossy(&output.stdout);
            //                     write_line(format!("{}", o), data_stream);

                                
            
            //                     write_line(format!("{} {}", DATA_SEND_C, DATA_SEND_M), s1);

            //                 }
            //                 _ => {}
            //             }
            //             break;
            //         }

                    
            //     }
            //     None => {}
            // }

            
            
        }

    }

}