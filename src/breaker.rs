
pub mod breaker{

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use ctrlc;

    pub struct EscapeHandler{
        running : Arc<AtomicBool>
    }


    



    impl EscapeHandler{

        pub fn new() -> EscapeHandler{
            //init du SIGINT handler pour arrêter le serveur de manière propre.
            let running = Arc::new(AtomicBool::new(false));
    
            let r = running.clone();
    
            ctrlc::set_handler(move || {
                r.store(true, Ordering::SeqCst);
            }).expect("Ertror setting Ctrl-C handler");
    
            return EscapeHandler{running};
        }


        pub fn escape(&mut self) -> bool{
            return self.running.load(Ordering::SeqCst);
        }
    }

    
}