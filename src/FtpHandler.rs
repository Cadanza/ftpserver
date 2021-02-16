#[path = "./code.rs"]
mod code;

#[path = "./messages.rs"]
mod messages;

pub mod ftp_handler{
    use super::code::code::*;
    use super::messages::messages::*;
    use std::net::TcpStream;
    use const_format::concatcp;

    pub struct FtpHandler{
        running : bool,
        data_port : Option<TcpStream>,
    }

    pub fn new() -> FtpHandler {
        return FtpHandler{running : true, data_port : None};
    }

    impl FtpHandler{

        

        /// # Handle request send by user and call the good function to response at the rquest
        pub fn request_handler(&mut self, mut data : Vec<&str>) -> (Code, Message){
                
            println!("{:?}", data);

            //let arg_pop : Option<&str> = data.pop();
            let command : &str = data[0];
            let arg_pop : Option<&str> = data.pop();


            println!("{}", command);

            match command{

                "USER" => return self.user_handler(arg_pop),

                "AUTH" => return (SESSION_NO_OPEN, AUTH_ERROR),
                
                "PASS" => return self.pass_handler(arg_pop),

                "QUIT" => return self.quit_handler(),

                //"LIST" => return self.list_handler(),

                "PASV" => return self.passiv_handler(),

                _ => return self.no_found_handler(),

            }
        }

        fn user_handler(&mut self, usm : Option<&str>) -> (Code, Message){
            
            if usm.is_none(){
                return (SYNTAX_ARGS_ERROR, UNVA_SYNTAX_ARGS);
            }

            let username : &str = usm.unwrap();
            let c : Code;
            let m : Message;

            println!("{}", username);
            
            if username == "anonymous"{
                c = NEED_PASSWORD;
                m = SPEC_PASSWORD;
            } else {
                c = SESSION_NO_OPEN;
                m = ANO_ONLY;
            }

            return (c, m);
        }

        fn pass_handler(&mut self, psw : Option<&str>) -> (Code, Message){

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

        fn no_found_handler(&mut self) -> (Code, Message) {
            return (UNKNONW_COMMAND_C, UNKNOWN_COMMAND_MES);
        }

        fn quit_handler(&mut self) -> (Code, Message){
            self.running = false;
            return (BYE, BYE_MES);
        }

        // fn list_handler(&mut self) -> (Code, Message){
        //     return  ;
        // }

        fn passiv_handler(&mut self) -> (Code, Message){
            const p1 : &str = "205";
            const p2 : &str = "179";
            let adr = format!("127.0.0.1:55055");

            let ret_m : Message = concatcp!(PASSIF_MODE_M, " (127,0,0,1,", p1, ",", p2, ")");          


            match TcpStream::connect(adr){
                Ok(stream) => {
                    println!("yes");
                    self.data_port = Some(stream);
                    return (PASSIF_MODE_C, ret_m);
                },
                //_ => return (BAD_COM_SEQ_C, BAD_COM_SEQ_M),
                Err(m) => {
                    println!("{:?}", m);
                    return (BAD_COM_SEQ_C, BAD_COM_SEQ_M);
                }
            }
        }

        pub fn running(&mut self) -> bool{
            return self.running;
        }

        
    }
    
    
    
}