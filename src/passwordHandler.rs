
#[path ="."]
pub mod password_handler{

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

    pub struct PasswordHandler {
        pub password : Option<String>,
        pub good_user : bool,
    } 

    impl PasswordHandler {

        pub fn execute(&self, stream : &mut TcpStream) -> bool{

            let c : Code;
            let m : Message;
            let pasw = &self.password;
            let good_psw : bool;

            if !self.good_user{
                c = SESSION_NO_OPEN_C;
                m = SESSION_NO_OPEN_M;
                good_psw = false;
            } else {

                match pasw {
                    Some(p) => {
    
                        if p == "anonymous" {
                            c = SESSION_OPEN_C;
                            m = SESSION_OPEN_M;
                            good_psw = true;
                        } else {
                            c = SESSION_NO_OPEN_C;
                            m = ANO_ONLY_M;
                            good_psw = false;
                        }
                    },
                    &None => {
                        c = UNVA_SYNTAX_ARGS_C;
                        m = UNVA_SYNTAX_ARGS_M;
                        good_psw = false;
                    }
                }

            }

            write_line(format!("{} {}", c, m), stream);

            return good_psw;
        }
    }

}