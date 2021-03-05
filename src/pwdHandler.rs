#[path ="."]

/// # Module to handle PWD ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod pwd_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "common.rs"]
    mod common;

    use std::net::TcpStream;
    use code::code::*;
    use common::common::*;

    /// # Structure to handle PWD command
    pub struct PwdHandler {

        /// actual path of user on server file system
        pub actual_path : String
    } 

    impl PwdHandler {

        /// Call when PWD command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream){
            let mut cut_actual_path : Vec<&str> = self.actual_path.split("/").collect();

            println!("{:?}", cut_actual_path);
            cut_actual_path.remove(0);
            cut_actual_path.remove(0);
            cut_actual_path.push("");
            let p : String = cut_actual_path.join("/");

            write_line(format!("{} {} is current directory", PATH_CREATED_C, p), stream);
        }
    }

}