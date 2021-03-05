
/// # Breaker object it's use to handle ctrl-c and close server
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
pub mod breaker{

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use ctrlc;

    /// # Structure to handle a SIGINT signal
    /// 
    /// # Parameters:
    /// - **running** *Arc<AtomicBool>* : 
    ///     - **true** if a SIGINT was recieve
    ///     - **false** default value
    /// 
    pub struct EscapeHandler{
        
        /// Contains boolean to check if we had recieve SIGINT or not
        running : Arc<AtomicBool>
    }

    impl EscapeHandler{

        /// Construct a new EscapeHandler 
        /// 
        /// Initialyze running variable and set function call when ctrl-c is detect
        /// 
        pub fn new() -> EscapeHandler{
            //init du SIGINT handler pour arrêter le serveur de manière propre.
            let running = Arc::new(AtomicBool::new(false));
    
            let r = running.clone();
    
            ctrlc::set_handler(move || {
                r.store(true, Ordering::SeqCst);
                log::info!("recieve SIG INT signal");
            }).expect("Ertror setting Ctrl-C handler");
    
            return EscapeHandler{running};
        }

        /// Returns the value of running variable
        pub fn escape(&mut self) -> bool{
            return self.running.load(Ordering::SeqCst);
        }
    }

    
}