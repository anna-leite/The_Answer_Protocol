use std::net::{TcpListener, TcpStream};
use protocol::error::ServerError;
use std::io::{BufRead, BufReader, Read, Write};

pub fn run_server() -> Result<(), ServerError> {
    let listener = create_listener("127.0.0.1:8080").map_err(ServerError::Network)?;
    println!("Server listening");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // creer uns struct avec toute les info client bien propre et tout balancer dans un thread?
                // log info ici ou dans le span qui sera creer avec le nouveau client?
                handle_client(stream); // gerer les retour en cas d'erreur grave (nettoyage ressources)
            },
            Err(e) => {
                // faire une gestion d'erreur plus precise dans le fichier error
                return Err(ServerError::Network(e));
            },
        }
        println!("Waiting for another connection ...")
    }

    Ok(())
}

fn create_listener(address: &str) -> Result<TcpListener, ServerError> {
    let listener = TcpListener::bind(address)
        .map_err(ServerError::Network)?;

    Ok(listener)
}

fn process_data(reader: &mut BufReader<TcpStream>, max_bytes: u64) -> Result<String, ServerError> {
    let mut buf = Vec::new();

    let read = reader
        .by_ref()
        .take(max_bytes)
        .read_until(b'\n', &mut buf)
        .map_err(ServerError::Network)?;

    // line too long, on rejette tout jusqu'au procahain \n
    if read as u64 == max_bytes && buf.last() != Some(&b'\n') {
        reader
            .skip_until(b'\n')
            .map_err(ServerError::Network)?;
        return Err(ServerError::LineTooLong);
    }

    // une ligne sans carctere de fin signifie que la connection s'est interompu 
    if buf.last() != Some(&b'\n') {
        return Err(ServerError::IncompleteLine);
    }

    String::from_utf8(buf)
        .map_err(ServerError::InvalidUTF8)
}

fn send_response(stream: &mut TcpStream, response: String) -> Result<(), ServerError> {
    let mut packet = response.into_bytes();
    if packet.last() != Some(&b'\n') {
        packet.push(b'\n');
    }

    stream.write_all(&packet).map_err(ServerError::Network)?;

    Ok(())
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

    let mut reader = BufReader::new(stream);

    // est ce le client peut gerer la limite d'octet d'une rep ?????
    loop {
        let command = match process_data(&mut reader, 1024) {
            Ok(com) => com,
            Err(err) => {
                todo!()
                // selon l'erreur on gere 
                // -> log 
                // et -> fermeture de la connection 
                // ou -> arret du server
            }
        }

        // on evoie la command au systeme du jeu
        // on recupere la reponse

        // on met le caractere de fin.
        let response = "en attendant".to_string();
        send_response(&mut stream, response).map_err(ServerError::Network)?;

        // envoi de la reponse 

    }

    Ok(())
}
