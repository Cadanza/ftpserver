#[path ="."]
/// Module to handle MKD ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod mkd_handler{

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

    /// # Structure to handle MDK command
    pub struct MkdHandler {
        pub session_open : bool,
        pub path : Option<String>,
        pub actual_path : String,
        pub root : String
    } 

    impl MkdHandler {

        /// Call when MKD command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - *stream** *TcpStream* stream use to send response to user
        /// 
        pub fn execute(&self, stream : &mut TcpStream){

            let c : Code;
            let m : Message;

            if !self.session_open {
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
            } else {

                match &self.path {
                    Some(dir) => {
                        match self.relative_to_absolute_path(format!("{}", dir)) {
                            Some(abs_dir) => {
                                if self.mkdir(abs_dir){
                                    log::info!("{} was correctly created", dir);
                                    c = PATH_CREATED_C;
                                    m = PATH_CREATED_M;
                                } else {
                                    c = FILE_NOT_ACCESS_C;
                                    m = FILE_NOT_ACCESS_M;
                                }
                            },
                            None => {
                                c = FILE_NOT_ACCESS_C;
                                m = FILE_NOT_ACCESS_M;
                            }
                        }
                    },
                    None => {
                        log::info!("No argument passed with this FTP Command");
                        c = UNVA_SYNTAX_ARGS_C;
                        m = UNVA_SYNTAX_ARGS_M;
                    }
                }

            }

            write_line(format!("{} {}", c, m), stream);
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
            log::info!("Try to go above the root folder");
                return None;
            }

            // construction of absolute path
            for d in cut_dir {
                if d != ""{
                    absolute_path.push(d);
                }
            }

            return Some(absolute_path.join("/"));

        }

        /// call mkdir unix command
        /// 
        /// # Arguments
        /// 
        /// - **dir** *String* : absolute directory path
        /// 
        /// # Returns
        /// 
        /// - *bool* : 
        ///     - *true* if command execute with success
        ///     - *false* else 
        fn mkdir(&self, dir : String) -> bool {
            let output = Command::new("mkdir").arg(dir).output().expect("failder to execute process");

            return  output.status.success();
        }
    
    }

    

}