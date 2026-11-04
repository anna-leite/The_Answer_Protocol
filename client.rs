use std::net::TcpStream;
use std::io::{Write, stdin, stdout, BufReader, prelude::*};

fn get_entry() -> String {
	let mut buf = String::new();

	stdin().read_line(&mut buf).expect("Reading error");
	buf.trim_end_matches('\n').trim_end_matches('\r').to_string()
}

fn read_server(stream: &TcpStream) {
	let mut buf = BufReader::new(stream.try_clone().unwrap());
	let mut response = String::new();

	match buf.read_line(&mut response) {
		Ok(0) | Err(_) => {
			println!("Loose connection with the server");
			return;
		}
		Ok(_) => {
			println!("Server says : {}", response);
		}
	}
}

fn exchange_with_server(mut stream: TcpStream) {
	let stdout = stdout();
	let mut io = stdout.lock();

	println!("Enter 'quit' when you want to leave");

	loop {
		write!(io, "> ").expect("Writing error");
		io.flush().expect("Flush error");

		let line = get_entry();

		if write!(stream, "{}\n", line).is_err() {
			eprintln!("Loose connection with the server");
			return;
		}

		if line == "quit" {
			println!("See you soon, bye !");
			return;
		}

		read_server(&stream);
	}
}
// 127.0.0.1 == localhost => same thing
fn main() {
	println!("Try to connect to server ...");
	match TcpStream::connect("127.0.0.1:8080") {
		Ok(stream) => {
			println!("Succes !");
			exchange_with_server(stream);
		}
		Err(e) => {
			eprintln!("Failed to establish connection: {}", e);
		}
	}
}