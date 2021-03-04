#[path ="."]
/// Module to handle RNFR ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod rnfr_handler{

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
    use std::fs::metadata;

    /// # Structure to handle MDK command
    pub struct RnfrHandler {
        pub session_open : bool,
        pub path : Option<String>,
        pub actual_path : String,
        pub root : String
    } 

    impl RnfrHandler {

        /// Call when RNFR command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream) -> Option<String> {

            let c : Code;
            let m : Message;

            let ret : Option<String>;

            if !self.session_open {
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
                ret = None;
            } else {

                match &self.path {
                    Some(dir) => {
                        match self.relative_to_absolute_path(format!("{}", dir)) {
                            Some(p) => {
                                c = FILE_SERV_WAIT_C;
                                m = FILE_SERV_WAIT_M;
                                ret = Some(format!("{}", p));
                            },
                            None => {
                                c = FILE_NOT_ACCESS_C;
                                m = FILE_NOT_ACCESS_M;
                                ret = None;
                            }
                        }
                    },
                    None => {
                        c = UNVA_SYNTAX_ARGS_C;
                        m = UNVA_SYNTAX_ARGS_M;
                        ret = None;
                    }
                }

            }

            write_line(format!("{} {}", c, m), stream);

            return ret;
        }

        /// Transform a relative path to an absolute path
        /// 
        /// # Arguments
        /// 
        /// - **dir** *String* : relative path to directory
        /// 
        /// # Returns
        /// 
        /// - *Option<String>*:
        ///     - *String* : absolute path of directory
        ///     - *None* : absolute path was above the root
        /// 
        fn relative_to_absolute_path(&self, dir : String) -> Option<String>{
            
            let mut cut_dir : Vec<&str> = dir.split("/").collect();
            let mut absolute_path : Vec<&str> = self.actual_path.split("/").collect();
            let mut cut_root : Vec<&str> = self.root.split("/").collect();

            absolute_path.pop();    //remove the last element ""
            cut_root.pop();         // remove the last element ""

            if cut_dir[0] == "."{   //if dir start with ./ we just remove . element
                cut_dir.remove(0);
                
            } else {    // else we remove all .. element and actualise the futur absolute path
                for d in cut_dir.clone() {
                    if d == ".."{
                        absolute_path.pop();
                        cut_dir.retain(|&x| x != d);
                    }
                }
            }
            
            
            if absolute_path.len() < cut_root.len() { //if we're under the root
                return None;
            }

            // construction of absolute path
            for d in cut_dir {
                if d != ""{
                    absolute_path.push(d);
                }
            }

            let ret_string : String = absolute_path.join("/");

            match metadata(absolute_path.join("/")){
                Ok(_) => {
                    return Some(ret_string);
                },
                Err(_) => {
                    return None;
                }
            }

        }
    
    }

    

}