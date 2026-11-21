use std::net::{TcpListener, TcpStream};
use protocol::error::ServerError;
use std::io::{BufRead, BufReader, Read, Write};
use std::io;
use std::thread;
use tracing::{error, warn, info};


pub fn run_server() -> Result<(), ServerError> {
    let listener = create_listener("127.0.0.1:8080").map_err(|e| {
        error!(
            error = %e,
            address = "127.0.0.1:8080",
            "Failed to create TCP listener"
        );
        ServerError::Network(e)
    })?;

    info!(
        address = "127.0.0.1:8080",
        "TCP server started successfully"
    )

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New client connection accepted");

                match thread::Builder::new().spawn(move || {
                    run_client_session(stream);
                }) {
                    Ok(_handle) => {
                        info!("Client handler thread spawed successfully");
                    },
                    Err(e) => {
                        warn!(
                            error = %e,
                            "Failed to span client handler thread. Connection will be droped"
                        );
                    }
                }
            },

            Err(e) => match e.king() {
                io::ErrorKind::Interrupted => {
                    warn!(error = %e, "Accept interrupted, retrying");
                    continue;
                }

                io::ErrorKind::ConnectionAborted => {
                    warn!(error = %e, "Incoming connection aborted");
                    continue;
                }

                io::ErrorKind::WouldBlock => {
                    warn!("No incoming connection available");
                    continue;
                }
            }

            _ => {
                error!(
                    error = %e,
                    "Unrecoverable error while accepting connection"
                );
                return Err(ServerError::Network(e));
            }
        }
    }

    info!("TCP server stopped");
    Ok(())
}

fn create_listener(address: &str) -> Result<TcpListener, ServerError> {
    let listener = TcpListener::bind(address)
        .map_err(ServerError::Network)?;

    Ok(listener)
}

fn process_data(reader: &mut BufReader<TcpStream>, max_bytes: u64) -> Result<Option<String>, ServerError> {
    let mut buf = Vec::new();

    let read = reader
        .by_ref()
        .take(max_bytes)
        .read_until(b'\n', &mut buf)
        .map_err(ServerError::Network)?;

    // no bytes read
    if read as u64 == 0 {
        return Ok(None);
    }

    // line too long
    if read as u64 == max_bytes && buf.last() != Some(&b'\n') {
        return Err(ServerError::LineTooLong);
    }

    // line without '\n'
    if buf.last() != Some(&b'\n') {
        return Err(ServerError::IncompleteLine);
    }

    String::from_utf8(buf)
        .map(Some)
        .map_err(ServerError::InvalidUTF8)
}

fn send_response(stream: &mut TcpStream, response: String) -> Result<(), ServerError> {
    // est ce que je gere un taille max de reponse???
    let mut packet = response.into_bytes();
    if packet.last() != Some(&b'\n') {
        packet.push(b'\n');
    }

    stream.write_all(&packet).map_err(ServerError::Network)?;

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<(), ServerError> {
    let mut writer = stream
        .try_clone()
        .map_err(ServerError::Network)?;
    let mut reader = BufReader::new(stream);

    loop {
        let command = match process_data(&mut reader, 1024)? {
            Some(command) => command,
            None => return Ok(()),
        }

        // log la command

        // on evoie la command au systeme du jeu
        // on recupere la reponse

        // log la response 
        let response = "en attendant".to_string();
        send_response(&mut writer, response)?;
    }

    Ok(())
}

fn run_client_session(stream: TcpStream) {
    let address = match stream.peer_addr() {
        Ok(address) => address,
        Err(err) => {
            error!("Cannot get client address: {}", err);
            return;
        }
    };

    let span = create_client_span(address.ip());

    let _guard = span.enter();

    info!("Client connected");

    match handle_client(stream) {
        Ok(()) => {
            info!("Client disconnected normally");
        },
        Err(ServerError::LineTooLong) => {
            warn!("Client disconnected: line too long");
        },
        Err(ServerError::IncompleteLine) => {
            warn!("Client disconnected: incomplete line");
        },
        Err(err) => {
            error!("Client session ended with error: {}", err);
        }
    }
}
