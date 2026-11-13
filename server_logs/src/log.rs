struct Log {
	timestamp: String,
	log_type: String,
	massage: String,
}

enum LogType {
	Info,
	Warn,
	Error,
}
// gestion de la sortie en fonction du type utilisation de writeln

impl LogType {
	pub fn to_string(&self) -> String {
		match self {
			Self::Info => "INFO".to_string(),
			Self::Warn => "WARN".to_string(),
			Self::Error => "ERROR".to_string(),
		}
	}
}

// ou alors Trois fonntion appele qui creer une phrase.