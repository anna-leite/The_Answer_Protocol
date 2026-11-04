enum Command {
	Connect, // CONNECT player_name ==> rep[OK, ERR 201 NAME_IN_USE]
	Look, // LOOK ==> rep[OK current room state as JSON]
	Move, // MOVE dirrection ==> rep[OK room=new_room, ERR 301 NO_EXIT]
	Quit, // QUIT ==> rep[OK bye]
	Chat, // CHAT SCOPE "message" ==> rep[OK, ERR 401 NOT_IN_GROUP]
	Who, // WHO ==> rep[OK players=nb_players]
	CreateGroup, // CREATE GROUP group_name ==> rep[OK group=group_name, ERR 201 NAME_IN_USE]
	GroupInvit, // GROUP INVITE group_name player_name ==> rep[OK, ERR 401 NOT_IN_GROUP, ERR 402 ALREADY_IN_GROUP]
	GroupJoin, // GROUP JOIN 


}

// ERR 901 SEND_FAILED == ATTENTION ERREUR POSSIBLE A CAHQUE ENVOI !!!!!!!!!!!!!!!

enum MessageScope {
	GLOBAL,
	ROOM,
	GROUP,
}

struct Request {
	command: String, // Seulement alpha ! non numerique!,
	arguments: String, // Seulement char 
}