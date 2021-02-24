#[path ="."]
/// Module to handle QUIT ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod port_handler{

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

    /// # Structure to handle PORT command
    pub struct PortHandler {

        /// arguments receive with command. It can be None if no arguments was found
        pub data : Option<String>,

        /// true if client session is open. If false, the command is not handle
        pub session_open : bool,
    } 

    impl PortHandler {

        /// Call when PORT command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        /// # Returns
        /// 
        /// - *(Option<u16>, Option<TcpStream>)*
        /// 
        ///     - *Option<u16>* : port envoyer par la commande
        /// 
        ///     - *Option<TcpStream> : Data stream open with information pass with command
        /// 
        pub fn execute(&self, stream : &mut TcpStream) -> (Option<u16>, Option<TcpStream>) {
            if !self.session_open {
                write_line(format!("{} {}", SESSION_NO_OPEN_C, SESSION_NO_OPEN_M), stream);
                return (None, None);
            } else {
                return self.handler(stream);
            }
        }   

        /// Call to handle data if session is open
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* stream use to send response to user
        /// 
        /// # Returns
        /// 
        /// - *(Option<u16>, Option<TcpStream>)*
        /// 
        ///     - *Option<u16>* : port envoyer par la commande
        /// 
        ///     - *Option<TcpStream> : Data stream open with information pass with command
        /// 
        fn handler(&self, stream : &mut TcpStream) -> (Option<u16>, Option<TcpStream>) {


            let port_ret : Option<u16>;
            let stream_ret : Option<TcpStream>;

            let c : Code;
            let m : Message;
            
            match &self.data {  //check if data exist
                
                Some(s) => { //data exist

                    match self.parse_data(s.to_string()) {  // try de get IPV4 address and socket port

                        Some(sa) => {   // IPV4 and socket port found

                            port_ret = Some(sa.0);

                            match self.open_stream(sa.1, sa.0){ // try to open stream
                                Some(strm) => { // stream open
                                    log::info!("Tcp data stream open at {} address", strm.peer_addr().unwrap());

                                    c = CONCLUD_COMMAND_C;
                                    m = CONCLUD_COMMAND_M;

                                    stream_ret = Some(strm);
                                },
                                None => {   // open socket failed
                                    stream_ret = None;
                                    
                                    c = SERVICE_UNVA_C;
                                    m = SERVICE_UNVA_M;
                                }
                            }
                            
                        },
                        None => {   // Get IPV4 or oprt has failed
                            
                            c = UNVA_SYNTAX_ARGS_C;
                            m = UNVA_SYNTAX_ARGS_M;

                            port_ret = None;
                            stream_ret = None;
                        }
                    }

                },
                None =>{    // No arguments was founded on command
                    
                    c = UNVA_SYNTAX_ARGS_C;
                    m = UNVA_SYNTAX_ARGS_M;

                    port_ret = None;
                    stream_ret = None;
                }
            }

            write_line(format!("{} {}", c, m), stream);

            return (port_ret, stream_ret);

        } 

        /// Use to transform arguments string and get tcp port and IPV4 adress
        /// 
        /// # Arguments
        /// 
        /// - **data** *String* : arguments recieve with command 
        /// 
        /// # Returns
        /// 
        /// - *Option<(u16, String)>*
        ///     - *u16* : tcp port 
        ///     - *String* : IPV4 adress
        ///     - *None* : Impossible to get port or adress
        /// 
        fn parse_data(&self, data : String) -> Option<(u16, String)>{

            let splited_data : Vec<&str> = data.split(",").collect();


            if splited_data.len() < 6 {
                return None;
            } else {
                let adr : String = format!("{}.{}.{}.{}", splited_data[0], splited_data[1], splited_data[2], splited_data[3]);
                let port : u16 = (256 * splited_data[4].parse::<u16>().unwrap() ) + splited_data[5].parse::<u16>().unwrap();
                
                return Some((port, adr));
            }
        }

        /// open tcp stream with previous adress information
        /// 
        /// # Arguments
        /// 
        /// - **adr** *String* : IPV4 adress
        /// - **port** *u16* : tcp socket port
        /// 
        /// # Returns
        ///  
        /// - *Option<TcpStream>* :
        ///     - *TcpStream* if stream is open
        ///     - *None* if connection was failed
        /// 
        fn open_stream(&self, adr : String, port : u16) -> Option<TcpStream> {

            match TcpStream::connect((adr, port)) {
                Ok(stream) => return Some(stream),
                _ => return None,
            }
        }
    }

}