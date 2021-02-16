#[path = "./code.rs"]
mod code;

#[path = "./messages.rs"]
mod messages;

pub mod ftp_handler{
    use super::code::code::*;
    use super::messages::messages::*;
    use std::net::TcpStream;

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

                "PASV" => return self.passiv_handler(&mut String::new()),

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

        fn passiv_handler(&mut self, s : & mut String) -> (Code, Message){

            let port : u16;
            let up1 : u16;
            let up2 : u16;

            let p = self.search_free_port();

            if p.is_none(){
                return (SERVICE_UNA_C, SERVICE_UNA_M);
            }

            self.data_port = Some(p.unwrap());
            port = self.data_port.as_ref().unwrap().local_addr().unwrap().port();
            up2 = port%256;
            up1 = (port-up2)/256;
            
            let m =format!("(127,0,0,1,{},{})", up1, up2);
            
            let ms = PASSIF_MODE_M;

            let rm : String = format!("{} {}", ms, m);


            s.push_str(&rm);

            return (PASSIF_MODE_C, &s[..]);
        }


        pub fn running(&mut self) -> bool{
            return self.running;
        }

        fn search_free_port(&mut self) -> Option<TcpStream>{
            for port in 1025..65535{
                match TcpStream::connect(("127.0.0.1", port)){
                    Ok(l) => return Some(l),
                    _ => {}
                }
            }

            return None;
        }


        
    }
    
    
    
}