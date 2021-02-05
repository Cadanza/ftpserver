


pub mod user{
    use std::net::TcpStream;
    use std::io::{Read, Write};
    use encoding::all::ASCII;
    use encoding::EncoderTrap;

    pub struct User{
        pub server_stream : TcpStream,
    }

    impl User{

        pub fn run(&mut self){            
            self.connect();
            loop{}
        }

        fn connect(&mut self){
            log::info!("220 FTP Server (Axolotl FTP)");
            let _ = self.server_stream.write(b"220 FTP Server (Axolotl FTP)").unwrap();
        }
    }

}