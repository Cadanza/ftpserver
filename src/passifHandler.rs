
#[path ="."]
/// # Module to handle PASV ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
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

        /// say if user session is open
        pub session_open : bool,
    } 

    impl PassivHandler {

        /// Call when  PASV is recieve by ftp handler
        /// 
        /// determine aa free tcp port and open listener to get data tcp stream
        /// 
        /// # Arguments
        /// 
        /// * `stream` *TcpStream* : stream to send request to user
        /// 
        /// # Returns
        /// 
        /// * `(Option<u16>, Option<TcpStream>)`
        ///     - Option<u16> : u16 if free port found, None else
        ///     - Option<TcpStream>: TcpStream if free port found and if client connect to data TcpListener
        /// 
        pub fn execute(&self, stream : &mut TcpStream) -> Option<TcpStream> {
            let port : Option<u16> = self.port_handler(stream);
            let stream : Option<TcpStream> = self.data_stream_handler(port);

            return stream;
        }

        /// Search a free tcp port and send port to client
        /// 
        /// # Arguments
        /// 
        /// * `stream` *TcpStream* : stream to send request to user
        /// 
        /// # Returns
        /// 
        /// * `Option<u16>` : 
        ///     - u16 if port was found 
        ///     - None if no free tcp port
        /// 
        fn port_handler(&self, stream : &mut TcpStream) -> Option<u16> {
            let up1 : u16;
            let up2 : u16;
            let port : Option<u16>;

            let c : Code;
            let m :  Message;
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
                        log::info!("Passiv Tcp address is : 127.0.0.1 : {}", p);
                    }
                    None => {
                        c = SERVICE_UNVA_C;
                        m = SERVICE_UNVA_M;
                        end = String::from("");
                        log::info!("No free Tcp port was found");
                    }
                }
            } else {
                log::info!("Ask access to server but user session is not open");
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
                end = String::from("");
                port = None;
            }

           write_line(format!("{} {}{}", c, m, end), stream);

           return port;
        }

        /// Listen tcp adress to get data tcp stream
        /// 
        /// # Arguments
        /// 
        /// - **port** *Option<u16>* port found previosly
        /// 
        /// # Returns 
        /// 
        /// - *Option<TcpStream>*
        ///     - *TcpStream* if client connect him to data listener
        ///     - *None* if port = None or client doesn't connect him to data listener
        /// 
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