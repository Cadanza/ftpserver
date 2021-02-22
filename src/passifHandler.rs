
#[path ="."]
pub mod passiv_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "common.rs"]
    mod common;


    use std::net::{TcpStream, TcpListener};
    use messages::messages::*;
    use code::code::*;
    use common::common::*;

    pub struct PassivHandler {
        pub session_open : bool,
    } 

    impl PassivHandler {
        pub fn execute(&self, stream : &mut TcpStream) -> (Option<u16>, Option<TcpStream>){
            let port : Option<u16> = self.port_handler(stream);
            let stream : Option<TcpStream> = self.data_stream_handler(port);

            return (port, stream);
        }


        fn port_handler(&self, stream : &mut TcpStream) -> Option<u16> {
            let up1 : u16;
            let up2 : u16;
            let port : Option<u16>;

            let c : Code;
            let m :  &str;
            let end : String;

            if self.session_open {

                port = search_free_port();

                match port {
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
            } else {
                c = SESSION_NO_OPEN;
                m = SESSION_NO_OPEN_MES;
                end = String::from("");
                port = None;
            }

           write_line(format!("{} {}{}", c, m, end), stream);

           return port;
        }

        fn data_stream_handler(&self, port : Option<u16>) -> Option<TcpStream> {

            let mut data_stream : Option<TcpStream> = None;

            match port {

                Some(port) => {
                    let l = TcpListener::bind(("127.0.0.1", port)).unwrap();

                    for ds in l.incoming(){
                        match ds {
                            Ok(stream) => data_stream = Some(stream),
                            _ => data_stream = None
                        }
                        break;
                    }
                },
                None => data_stream = None,
                
            }

            return data_stream;
        }

    }

}