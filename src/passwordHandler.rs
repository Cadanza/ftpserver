
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
    } 

    impl PasswordHandler {

        pub fn execute(&self, stream : &mut TcpStream) -> bool{

            let c : Code;
            let m : &str;
            let pasw = &self.password;
            let good_psw : bool;

            match pasw {
                Some(p) => {

                    if p == "anonymous" {
                        c = SESSION_OPEN;
                        m = SESSION_OPEN_MES;
                        good_psw = true;
                    } else {
                        c = SESSION_NO_OPEN;
                        m = ANO_ONLY;
                        good_psw = false;
                    }
                },
                &None => {
                    c = SYNTAX_ARGS_ERROR;
                    m = UNVA_SYNTAX_ARGS;
                    good_psw = false;
                }
            }
                        
            write_line(format!("{} {}", c, m), stream);

            return good_psw;
        }
    }

}