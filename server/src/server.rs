use std::net::TcpListener;
use protocol::error::ServerError;

pub fn run_server(listener: TcpListener) -> Result<(), ServerError> {
    println!("Server listening");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // log info ici ou dans le span qui sera creer avec le nouveau client?
                drop(stream);
            },
            Err(e) => {
                // faire une gestion d'erreur plus precise dans le fichier error
                return Err(ServerError::Network(error));
            },
        }
        println!("Waiting for another connection ...")
    }

    Ok(())
}

pub fn create_listener(address: &str) -> Result<TcpListener, ServerError> {
    let listener = TcpListener::bind(address)
        .map_err(ServerError::Network)?;
    
    Ok(listener)
}

fn handle_client(stream: TcpStream) -> Result<(), ServerError> {
    let address = stream.peer_addr()
        .map_err(ServerError::Network)?;

    let span = tracing::info_span!(
        "tcp_connection",
        client_ip = %address.ip(),
    );

    let _guard = span.enter();

    tracing::info!("Client connected");

    let mut buf = BufReader::new(stream);
    let mut packet = String::new();

    loop {
        packet.clear()
        // ecrire process data ??
//         Lire les octets disponibles avec fill_buf().



// Chercher le caractère de fin de ligne (\n).



// Vérifier la limite : si l'ajout des octets dépasse 1024, rejeter le packet.



// Ajouter les octets au buffer du packet avec extend_from_slice().



// Consommer les octets traités avec consume().



// Recommencer tant que le caractère de fin de ligne n'a pas été trouvé.



// Une fois la ligne complète, vérifier que les octets constituent du UTF-8 valide avec from_utf8().

        // lire une limite de 1024 octets

    }

    Ok(())
}
