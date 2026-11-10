// trois types: OK ERR EVT

enum Type {
	OK {message: }, // OK ARG(si besoin)[LF] // a envoyer au client 
	ERR {message: CommandError}, // ERR NUMBER NAME[LF] // a envoyer au client 



	EVT{message: Event}, // EVT EVENT_TYPE EVENT_DATA[LF] // message asynchrone 
}