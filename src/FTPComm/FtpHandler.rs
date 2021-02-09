pub mod FtpHandler{
    use std::sync::{Mutex, Arc};
    use std::fs::File;
    use std::io::{prelude::*, BufRead};
    use std::collections::HashMap;

    struct FtpHandler{
        user_password : HashMap<&str, &str>,
        excpeted_password : &str
    }

    impl FtpHandler{

        pub fn new() -> FtpHandler{

        }


        fn load_password() -> HashMap<&str, &str>{
            let up = HashMap<&str, &str>::new();
    
            let mutex = Arc::new(Mutex::new(File::create("../userInfo")));
            let c_mutex = Arc::clone(&mutex);
    
            // try to lock mutex
            do{
                let mut lock = c_mutex.try_lock();
            } while(let Ok(ref mut mutex) = lock)
    
            //read file
            let file : File = **mutex;
            let lines = BufReader::new(file).lines();
    
            // unlock mutex
            drop(mutex);
            drop(c_mutex);
    
            let mut splited_line;
    
            for line in lines {
                splited_line = line.split(" ");
                up.insert(splited_line[0], splited_line[1]);
            }       
            
            return ud;
        }
    
        pub fn request_handler(mut data : Vec<&str>){
            let command : &str = data.pop();
    
            match command{
    
                "USER" => 
    
    
            }
        }
    }

    
}