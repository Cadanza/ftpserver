
#[path = "."]
pub mod user_handler{

    #[path = "command.rs"]
    mod command;

    #[path = "code.rs"]
    mod code;

    #[path = "messages.rs"]
    mod messages;

    use std::net::TcpStream;
    use command::command::*;
    use code::code::*;
    use messages::messages::*;


    pub struct UserHandler {
        pub username : Option<String>,
    }

    impl UserHandler{
        

        pub fn execute(&self, stream: &mut TcpStream) {
            let usm = &self.username;
            let c : Code;
            let m : &str;


            match usm {
                Some(s) => {
                    let name : String = s.to_string();
                    if name == "anonymous"{
                        c = NEED_PASSWORD;
                        m = SPEC_PASSWORD;
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

            write_line(format!("{}{}", c, m), stream);
        }
        
    }


}