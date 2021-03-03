
#[path ="."]
/// # Module to handle LIST ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
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

    /// # Structure to handle the LIST command
    /// 
    /// # Parameters
    /// 
    /// * `data_stream` *Option<TcpStream>* tcp stream to send data
    /// 
    /// * `session_open` *bool* true if session is open, false else.
    pub struct ListHandler {

        /// stream uise to send result of command
        /// - *TcpStream* if a free port and client connexion was found
        /// - *None* else
        pub data_stream : Option<TcpStream>,

        /// if session is not open, do nothing
        pub session_open : bool,

        pub path : String,
    } 

    impl ListHandler {

        /// Call when LIST command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* tcp stream to send request to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream){

            if !self.session_open {
                write_line(format!("{} {}", SESSION_NO_OPEN_C, SESSION_NO_OPEN_M), stream);
            } else {
                self.send_data(stream);
            }

        }

        /// Send data to user
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* tcp stream to send data to user
        /// 
        fn send_data(&self, stream : &mut TcpStream) {

            match &self.data_stream {
                Some(dts) => {

                    let mut s = dts.try_clone().unwrap();

                    write_line(format!("{} {}", DATA_COME_C, DATA_COME_M), stream);
                    
                    
                    match self.get_command_res() {
                        Some(data) => {
                            write_data(data, &mut s);
                        },
                        None => {

                        }
                    }

                    write_line(format!("{} {}", DATA_SEND_C, DATA_SEND_M), stream);

                },
                None => {
                    write_line(format!("{} {}", DATA_STREAM_ERROR_C, DATA_STREAM_ERROR_M), stream);
                }
            }
        }

        /// Get result of ls -n command
        /// 
        /// # Returns
        /// 
        /// - *String* result of ls -n convert to string
        /// 
        fn get_command_res(&self) -> Option<String> {
            let output = Command::new("ls").arg(format!("{}",self.path)).arg("-n").output().expect("failder to execute process");

            if output.status.success(){ 
                let o = String::from_utf8_lossy(&output.stdout);

                return Some(format!("{}", o));
            } else {
                log::info!("ls {} -n failed\n{}", self.path, String::from_utf8_lossy(&output.stderr));
                return None;
            }
                    
            
        }

    }

}