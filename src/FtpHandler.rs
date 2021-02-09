#[path = "./code.rs"]
mod code;

#[path = "./messages.rs"]
mod messages;

pub mod FtpHandler{
    use super::code::code::*;
    use super::messages::messages::*;

    
    /// # Handle request send by user and call the good function to response at the rquest
    pub fn request_handler(mut data : Vec<&str>) -> (Code, Message){
        let ret_pop : Option<&str> = data.pop();

        if ret_pop.is_none(){
            return (SYNTAX_COMMAND_ERROR, UNVA_SYNTAX_COMMAND);
        }
        let command : &str = ret_pop.unwrap();

        match command{

            "USER" => return user_handler(data.pop()),

            "AUTH" => return (SESSION_NO_OPEN, AUTH_ERROR),
            
            "PASS" => return pass_handler(data.pop()),

            _ => return no_found_handler(),

        }
    }

    fn user_handler(usm : Option<&str>) -> (Code, Message){
        
        if usm.is_none(){
            return (SYNTAX_ARGS_ERROR, UNVA_SYNTAX_ARGS);
        }

        let username : &str = usm.unwrap();
        let c : Code;
        let m : Message;

        if username == "anonymous"{
            c = NEED_PASSWORD;
            m = SPEC_PASSWORD;
        } else {
            c = SESSION_NO_OPEN;
            m = ANO_ONLY;
        }

        return (c, m);
    }

    fn pass_handler(psw : Option<&str>) -> (Code, Message){

        if psw.is_none(){
            return (SYNTAX_ARGS_ERROR, UNVA_SYNTAX_ARGS);
        }

        let password : &str = psw.unwrap();
        let c : Code;
        let m : Message;

        if password == "anonymous" {
            c = SESSION_OPEN;
            m = SESSION_OPEN_MES;
        } else {
            c = SESSION_NO_OPEN;
            m = ANO_ONLY;
        }

        return (c, m);
    }
    
    fn no_found_handler() -> (Code, Message) {
        return (UNKNONW_COMMAND_C, UNKNOWN_COMMAND_MES);
    }
    
}