
#[path = "."]
pub mod user_handler{

    #[path = "common.rs"]
    mod common;

    #[path = "code.rs"]
    mod code;

    #[path = "messages.rs"]
    mod messages;

    use std::net::TcpStream;
    use common::common::*;
    use code::code::*;
    use messages::messages::*;


    pub struct UserHandler {
        pub username : Option<String>,
    }

    impl UserHandler{
        

        pub fn execute(&self, stream: &mut TcpStream) -> bool {
            let usm = &self.username;
            let c : Code;
            let m : &str;
            let good_user : bool;


            match usm {
                Some(s) => {
                    let name : String = s.to_string();
                    if name == "anonymous"{
                        c = NEED_PASSWORD;
                        m = SPEC_PASSWORD;
                        good_user = true;
                    } else {
                        c = SESSION_NO_OPEN;
                        m = ANO_ONLY;
                        good_user = false;
                    }

                },
                &None => {
                    c = SYNTAX_ARGS_ERROR;
                    m = UNVA_SYNTAX_ARGS;
                    good_user = false;
                }
            }

            write_line(format!("{}{}", c, m), stream);

            return good_user;
        }
        
    }


}