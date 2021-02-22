
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
            let m : Message;
            let good_user : bool;


            match usm {
                Some(s) => {
                    let name : String = s.to_string();
                    if name == "anonymous"{
                        c = SPEC_PASSWORD_C;
                        m = SPEC_PASSWORD_M;
                        good_user = true;
                    } else {
                        c = SESSION_NO_OPEN_C;
                        m = ANO_ONLY_M;
                        good_user = false;
                    }

                },
                &None => {
                    c = UNVA_SYNTAX_ARGS_C;
                    m = UNVA_SYNTAX_ARGS_M;
                    good_user = false;
                }
            }

            write_line(format!("{}{}", c, m), stream);

            return good_user;
        }
        
    }


}