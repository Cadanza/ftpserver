    
#[allow(dead_code)]
/// # Common function use by many module
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod common {
    use std::net::TcpStream;
    use std::net::TcpListener;
    use std::io::{BufRead, BufReader, Write};
    
    /// # Send request to user
    /// 
    /// # Arguments
    /// 
    /// - **message** *String* message to send to user
    /// - **stream** *TcpStream* stream to communicate with user
    /// 
    pub fn write_line(message : String, stream : &mut TcpStream){

        let req : &str =  &*format!("{}\n", message);
    
        let tcp_req : &[u8] = req.as_bytes();
    
        log::info!("send => {}", message);
    
        stream.write(tcp_req).unwrap();
    
    }

    /// # Send data to user
    /// 
    /// # Arguments
    /// 
    /// - **data** *String* data to send to user
    /// - **stream** *TcpStream* data stream to communicate with user
    /// 
    pub fn write_data(data : String, stream : &mut TcpStream){
        let req : &str =  &*format!("{}", data);
    
        let tcp_req : &[u8] = req.as_bytes();
    
        //log::info!("send => {}", message);
    
        stream.write(tcp_req).unwrap();
    }
    
    /// # Search free Tcp Port
    /// 
    /// Uses for passiv mode
    /// 
    /// # Returns
    /// 
    /// Option<u16> :
    ///     - *u16* : port found
    ///     - *None* : no port found
    /// 
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
