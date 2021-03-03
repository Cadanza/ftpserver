mod user;
mod breaker;

use log::LevelFilter;
use std::{env, assert, thread};
use std::net::TcpListener;
use user::user::User;
use breaker::breaker::EscapeHandler;
use std::sync::mpsc::{self};
///
/// # main function of server who handle users connexion
/// 
/// * author : Saulquin Clément/Aurélie
/// * version : 1.0
/// 
/// - Initialize TCP communication
/// - Listen all connexion:
///     - when a new user is connect, main function launch user thread
/// 
fn main() {

    //récupération des arguments passé en ligne de commande
    let args : Vec<String> = env::args().collect();


    // petit endoirt pour tester plus rapidement mes commandes
    if args.len() == 2 {
        return;
    }

    assert!(args.len()>=3);

    //init des variables locales dont les valeurs sont passé par paramètre.
    
    

    let mut cut_root : Vec<&str> = args[1].split("/").collect();

    if cut_root.last() != Some(&"") {
        cut_root.push("");
    }
    let file_root : &String = &cut_root.join("/");

    let port : &String = &args[2];

    let addr : &str = &*format!("127.0.0.1:{}", port);  // adresse du serveur

    // init du fichier de log
    simple_logging::log_to_file("server.log", LevelFilter::Info).unwrap();


    let mut handler = EscapeHandler::new();
    
    // stock les threads  pour les fermés proprement à la fermeture du serveur.
    let mut users : Vec<(thread::JoinHandle<()>, mpsc::Sender<bool>) > = vec![];
    
    //init du TCP listener
    let listener = TcpListener::bind(addr).unwrap();

    // on le met en mode non bloquant pour pouvoir sortir de la boucle en cas de SIGINT
    listener.set_nonblocking(true).unwrap();

    log::info!("Server Open");


    println!("Welcome to Axolotl FTP Server");

    

    // reception des demandes de collections.
    for stream in listener.incoming() {

        match stream{   //si qq se connectte
            Ok(stream) => {
                
                let (s, r): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();

                log::info!("new connection {}",stream.peer_addr().unwrap() );   // on inscrit le connection dans les log

                let mut u = User{ server_stream : stream, stop : r, path : file_root.to_string()};  // création de l'utilisateur

                let t = thread::spawn(move || { //lancement du thread
                    u.run()
                });

                users.push((t, s));  // on rajoute le thread
                
            }
            Err(_) => {
            }
        }   

        // si un SIGINT est repéré, on arrête de regarder les connections sur le ports.
        if handler.escape(){
            break;
        }

    }
    
    //fermeture du serveur
    drop(listener);

    for (jh, sd) in users{
        match sd.send(true) {
            Ok(_) => jh.join().unwrap(),
            Err(_) => {}
        }
        
        
    }
    
    println!("Server close");

    log::info!("server close");

}

