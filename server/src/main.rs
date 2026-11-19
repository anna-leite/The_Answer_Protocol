use protocol::command::{create_response};
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream, address: &str){
	// PREMIERE SOLUTION ---------------------------------------------
	// let mut buffer = [0; 1024];
	// stream.read(&mut buffer).expect("Failed to read from client!");

	// let request = String::from_utf8_lossy(&buffer[..]);
	// println!("Received request: {}", request);
	// let response = "Hello, Client!".as_bytes();
	// stream.write(response).expect("Failed to write response!");

	// DEUXIEME SOLUTION --------------------------------------------
	// if let Err(e) = stream.write_all(b"Hello, Client!\n") {
	// 				println!("Failed to send message to client {} : {}", address, e);
	// 				return;
	// 			}

	let mut buf = BufReader::new(stream.try_clone().unwrap());
	let mut msg = String::new();

	loop {
		msg.clear();

		match buf.read_line(&mut msg) {
			Ok(0) => {
				println!("Client disconnected {}", address);
				return;
			}
			Ok(_) => {
				let answer = match create_response(msg.clone()) {
					Ok(response) => response,
					Err(_error) => {
						println!("Server error");
						// ici on peut envoyer le log d'erreur
						// ici on peut envoyer le log de deconnexion du client 
						return; // arret de la connexion car probleme de server
					}
				};

				println!("message from {} : {}", address, answer);

				let answer = format!("{}\n", answer);


				if let Err(e) = stream.write_all(answer.as_bytes()) {
					println!("Failed to send message to client {} : {}", address, e);
					return;
				}
			}
			Err(e) => {
				eprintln!("Failed reading for the client {} : {}", address, e);
				return;
			}
		}
	}


	// let response = "Goodbye, Client!".as_bytes();
	// stream.write(response).expect("Failed to write response!");
}


fn main(){
	let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to adress");
	let mut handles = Vec::new();


	println!("Server listening on 127.0.0.1:8080");
	for stream in listener.incoming(){
		match stream{
			Ok(stream) => {
				let address = match stream.peer_addr() { // adresse IP + port et pour avoir que l'adresse ip : il suffit d'utiliser ip() sur peer_addr()
					Ok(addr) => format!("[adress : {}]", addr),
					Err(_) => "unknown".to_owned()
				};

				println!("New client {}", address);
				handles.push(std::thread::spawn(move|| handle_client(stream, &address)));
			}
			Err(e) => {
				eprintln!("Failed to establish connection: {}", e);
			}
		}
		println!("Waiting for another client...");
	}

	for h in handles {
		h.join().expect("Failed 'join'");
	}
	// Gerer exit proprement avec une recuperation de input user!
}

// . Ce qu'il faut ajouter
// Pour ton serveur, les protections importantes sont :
// Protection	Rôle
// Limite de taille	Empêcher les lignes excessivement longues
// Validation UTF-8	Rejeter les données qui ne sont pas du texte UTF-8 valide
// Validation des commandes	Rejeter les commandes malformées
// Vérification de l'état	Empêcher les commandes non autorisées avant authentification
// Gestion des erreurs	Éviter les panic en cas de données inattendues

// En résumé : BufReader gère efficacement la lecture tamponnée et write_all() l'écriture complète. Ce sont des outils de communication, pas des mécanismes de sécurité.

// Risque	                        Protection
// Ligne gigantesque	            Limite de 1024 octets
// Ligne jamais terminée	        Délai maximal de réception
// Trop de commandes successives	Limitation du nombre de commandes par période
// Données UTF-8 invalides	        Validation avant parsing
// Client qui se déconnecte	        Gestion de read() retournant 0
// Trop de connexions simultanées	Limitation des connexions

fn main {
	// on met en place le system pour les logs 
	// on met en place le thread pour recuperer l'input utilisateur grace a un recupe client
		// est ce que ce thread peut permettre de filtrer les logs d'un autre maniere??
	// run server 
	// match err
	// on ferme le server en netoyant bien les ressources.
}