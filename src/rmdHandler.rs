#[path ="."]
/// Module to handle RMD ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod rmd_handler{

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
    use std::fs::metadata;

    /// # Structure to handle MDK command
    pub struct RmdHandler {
        pub session_open : bool,
        pub path : Option<String>,
        pub actual_path : String,
        pub root : String
    } 

    impl RmdHandler {

        /// Call when RMD command is recieve by ftp handler
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
                                if self.rmrf(abs_dir){
                                    c = PATH_REMOVED_C;
                                    m = PATH_REMOVED_M;
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
                return None;
            }

            // construction of absolute path
            for d in cut_dir {
                if d != ""{
                    absolute_path.push(d);
                }
            }

            let ret_string : String = absolute_path.join("/");

            if metadata(absolute_path.join("/")).unwrap().is_dir(){
                return Some(ret_string);
            }

            return None;

        }

        /// call rm -rf unix command
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
        fn rmrf(&self, dir : String) -> bool {
            let output = Command::new("rm").arg("-rf").arg(dir).output().expect("failder to execute process");

            return  output.status.success();
        }
    
    }

    

}