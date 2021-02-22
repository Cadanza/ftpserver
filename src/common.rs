    
#[allow(dead_code)]
pub mod common {
    use std::net::TcpStream;
    use std::net::TcpListener;
    use std::io::Write;


    pub fn write_line(message : String, stream : &mut TcpStream){

        let req : &str =  &*format!("{}\n", message);
    
        let tcp_req : &[u8] = req.as_bytes();
    
        log::info!("send => {}", message);
    
        stream.write(tcp_req).unwrap();
    
    }

    pub fn write_data(data : String, stream : &mut TcpStream){
        let req : &str =  &*format!("{}", data);
    
        let tcp_req : &[u8] = req.as_bytes();
    
        //log::info!("send => {}", message);
    
        stream.write(tcp_req).unwrap();
    }

    pub fn search_free_port() -> Option<u16>{
        for port in 1025..65535{
            match TcpListener::bind(("127.0.0.1", port)){
                Ok(l) => {
                    drop(l);
                    return Some(port);
                },
                _ => {}
            }
        }

        return None;
    }


}
