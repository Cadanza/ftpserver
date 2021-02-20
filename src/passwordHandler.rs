
#[path ="."]
pub mod password_handler{

    #[path = "messages.rs"]
    mod messages;

    #[path = "code.rs"]
    mod code;

    #[path = "command.rs"]
    mod command;

    use std::net::TcpStream;
    use messages::messages::*;
    use code::code::*;
    use command::command::*;

    pub struct PasswordHandler {
        pub password : Option<String>,
    } 

    impl PasswordHandler {

        pub fn execute(&self, stream : &mut TcpStream){

            let c : Code;
            let m : &str;
            let pasw = &self.password;

            match pasw {
                Some(p) => {

                    if p == "anonymous" {
                        c = SESSION_OPEN;
                        m = SESSION_OPEN_MES;
                    } else {
                        c = SESSION_NO_OPEN;
                        m = ANO_ONLY;
                    }
                },
                &None => {
                    c = SYNTAX_ARGS_ERROR;
                    m = UNVA_SYNTAX_ARGS;
                }
            }
                        
            write_line(format!("{} {}", c, m), stream);
            

        }
    }

}