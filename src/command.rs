    
#[allow(dead_code)]
pub mod command {
    use std::net::TcpStream;

    use std::io::Write;

   
    pub type Cmd = Box<dyn FtpCommand>;

    pub trait FtpCommand {
        fn execute(&self, stream : TcpStream);
    }

    pub fn write_line(message : String, stream : TcpStream){

        let mut s = stream.try_clone().unwrap();
        let req : &str =  &*format!("{}\n", message);
    
        let tcp_req : &[u8] = req.as_bytes();
    
        log::info!("send => {}", message);
    
        s.write(tcp_req).unwrap();
    
        drop(s);
    }

}
