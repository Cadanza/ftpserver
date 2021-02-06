mod user;

use log::LevelFilter;
use std::env;
use std::assert;
use std::thread;
use std::net::TcpListener;
use user::user::User;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;



fn main() {

    //récupération des arguments passé en ligne de commande
    let args : Vec<String> = env::args().collect();

    assert!(args.len()>=3);

    //init des variables locales dont les valeurs sont passé par paramètre.
    let _file_root : &String = &args[1];
    
    let port : &String = &args[2];

    let addr : &str = &*format!("127.0.0.1:{}", port);  // adresse du serveur

    // init du fichier de log
    let _logger = simple_logging::log_to_file("server.log", LevelFilter::Info);

    //init du SIGINT handler pour arrêter le serveur de manière propre.
    let running = Arc::new(AtomicBool::new(false));

    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("Ertror setting Ctrl-C handler");
    
    // stock les threads  pour les fermés proprement à la fermeture du serveur.
    let mut users : Vec<thread::JoinHandle<()>> = vec![];
    
    //init du TCP listener
    let listener = TcpListener::bind(addr).unwrap();

    // on le met en mode non bloquant pour pouvoir sortir de la boucle en cas de SIGINT
    listener.set_nonblocking(true).unwrap();

    // reception des demandes de collections.
    for stream in listener.incoming() {

        match stream{   //si qq se connectte
            Ok(stream) => {

                log::info!("new connection {}",stream.peer_addr().unwrap() );   // on inscrit le connection dans les log

                let mut u = User{ server_stream : stream};  // création de l'utilisateur

                let t = thread::spawn(move || { //lancement du thread
                    u.run()
                });

                users.push(t);  // on rajoute le thread
                
            }
            Err(_) => {
            }
        }   

        // si un SIGINT est repéré, on arrête de regarder les connections sur le ports.
        if running.load(Ordering::SeqCst){
            break;
        }

    }
    

    //fermeture du serveur
    drop(listener);

    for tr in users{
        tr.join().unwrap();
    }
    
    println!("Server close");

    log::info!("server close");

}

