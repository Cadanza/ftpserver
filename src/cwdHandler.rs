#[path ="."]
/// Module to handle CWD ftp command
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod cwd_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "common.rs"]
    mod common;

    #[path = "cdupHandler.rs"]
    mod cdup_handler;

    use std::net::TcpStream;
    use messages::messages::*;
    use code::code::*;
    use common::common::*;
    use std::process::Command;
    use std::iter::FromIterator;
    use cdup_handler::cdup_handler::*;

    /// # Structure to handle CWD command
    pub struct CwdHandler {

        /// true if session is correctly open
        pub session_open : bool,

        /// actual path location of current user
        pub actual_path : String,

        /// directory where user want to go
        /// It can be *None* if no argument was founded after CWD keyword
        pub directory : Option<String>,

        pub root : String
    } 

    impl CwdHandler {

        /// Call when CWD command is recieve by ftp handler
        /// 
        /// # Arguments
        /// 
        /// - **stream** *TcpStream* stream use to send response to user
        /// 
        /// # Returns
        /// 
        /// - *String* : new actual path of user
        /// 
        pub fn execute(&self, stream : &mut TcpStream) -> String {

            let ret : String;

            let c : Code;
            let m : Message;

            match &self.directory {
                Some(dir) => {

                    if dir == ".."{
                        return CdupHandler{
                            session_open : self.session_open,
                            actual_path : format!("{}" ,self.actual_path),
                            root : format!("{}", self.root)
                        }.execute(stream);
                    }

                    let mut clean_dir = self.clear_dir_name(dir.to_string());

                    if self.check_if_dir_exist(&mut clean_dir) {
                        c = FILE_SERVICE_FINISH_C;
                        m = FILE_SERVICE_FINISH_M;
                        ret = format!("{}{}/", self.actual_path, clean_dir);
                    } else {
                        c = FILE_NOT_ACCESS_C;
                        m = FILE_NOT_ACCESS_M;
                        ret = format!("{}", self.actual_path);
                    }
                } None => {
                    c = UNVA_SYNTAX_ARGS_C;
                    m = UNVA_SYNTAX_ARGS_M;
                    ret = format!("{}", self.actual_path);
                }
            }

            write_line(format!("{} {}", c, m), stream);


            return ret;
        }

        /// Call by execute function to know if dir exists and also if desired directory is a directory and not a file
        /// 
        /// # Arguments
        /// 
        /// - **dir** *String* desired directory
        /// 
        /// # Returns
        /// 
        /// - *bool* :
        ///     - **true** if dir exist and dir is directory
        ///     - **false** else
        /// 
        fn check_if_dir_exist(&self, dir : &mut String) -> bool {

                    
            let o = Command::new("ls")
                .arg("-n")
                .arg(format!("{}/",self.actual_path))
                .output()
                .expect("failder to execute process");

            for s in String::from_utf8_lossy(&o.stdout).lines() {

                if s.chars().next() == Some('d'){
                    let l : Vec<&str> = s.split(" ").collect();
                    if *dir == String::from(*l.last().unwrap()){
                        return true;
                    }

                }
            }

            return false;
        }

        /// remove some characthers at the beginning of dir
        /// The objectif is to have path without / or ./ at the begenning
        /// 
        /// # Arguments
        /// 
        /// - **dir** *String* : directory to parse
        /// 
        /// # Returns
        /// 
        /// *String* : directory without ./ or /
        /// 
        fn clear_dir_name(&self, dir : String) -> String {
            let mut s : Vec<char> = dir.chars().collect();

            if s[0] == '.'{
                s.remove(0);
            }

            if s[0] == '/'{
                s.remove(0);
            }

            return String::from_iter(s);
        }
    }

}