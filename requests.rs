enum Command {
	Connect {name: String}, // CONNECT player_name ==> rep[OK, ERR 201 NAME_IN_USE]
	Look, // LOOK ==> rep[OK current room state as JSON]
	Move {direction: Direction}, // MOVE dirrection ==> rep[OK room=new_room, ERR 301 NO_EXIT]
	Quit, // QUIT ==> rep[OK bye]
	Chat {scope: MessageScope, message: String}, // CHAT SCOPE "message" ==> rep[OK, ERR 401 NOT_IN_GROUP]
	Who, // WHO ==> rep[OK players=nb_players]
	CreateGroup {name: String}, // CREATE GROUP group_name ==> rep[OK group=group_name, ERR 201 NAME_IN_USE]
	GroupInvit {group_name: String, player_name: String}, // GROUP INVITE group_name player_name ==> rep[OK, ERR 401 NOT_IN_GROUP, ERR 402 ALREADY_IN_GROUP, ERR 403 GROUP_NOT_FOUND, ERR 202 NAME_NOT_FOUND]
	GroupJoin {name: String}, // GROUP JOIN group_name ==> rep[OK group=group_name, ERR 403 GROUP_NOT_FOUND, ERR 402 ALREADY_IN_GROUP]
	GroupLeave {name: String}, // GROUP LEAVE group_name ==> rep[OK, ERR 403 GROUP_NOT_FOUND]
	Take {item: String}, // TAKE item_name ==> rep[OK taken=item_name, ERR 404 ITEM_NOT_FOUND]
	Drop {item: String}, // DROP item_name ==> rep[OK dropped=item_name, ERR 404 ITEM_NOT_IN_INVENTORY]
	Inventory,  // INVENTORY ==> rep[OK [current items as JSON array]]
	Talk {name: String}, // TALK ncp_name ==> rep[OK "ncp_message", ERR 404 NPC_NOT_FOUND]
	Attack {name: String}, // ATTACK ncp_name ==> rep[OK combat_status_as_JSON, ERR 404 NPC_NOT_FOUND, ERR 405 NPC_NOT_HOSTILE]
	Status, // STATUS ==>  rep[OK {"hp": , "max_hp": , "status": ""}]
	// ATTENTION D4AUTRE COMMANDES A IMPLEMENTER POUR LES COMBATS
	Quest, // QUEST ncp_name ==> rep[OK quest_data_as_json, ERR 404 NPC_NOT_FOUND, ERR 406 NO_QUEST_AVAILABLE]
	Quests, // QUESTS ==> rep[OK list_all_player_quest_data_as_JSON_array]
	// ATTENTION D4AUTRE COMMANDES A IMPLEMENTER POUR LES QUESTS
	Help, // HELP optional_command_name ==> rep[OK rules, ERR 101 COMMAND_NOT_FOUND]

}

// implementer les methodes qui renvoie les str pour faire le paquets en UTF-8

// ERR 901 SEND_FAILED == ATTENTION ERREUR POSSIBLE A CAHQUE ENVOI !!!!!!!!!!!!!!!
// CREER DES NOUVELLE ERREUR POUR LES NOM DE GROUPE PAS TROUVE !!!!!!!!!!!!!!!!!!
//	== ERR 403 GROUP_NOT_FOUND
//	== ERR 202 NAME_NOT_FOUND
//	== ERR 101 COMMAND_NOT_FOUND

enum Direction {
	Nord,
	Sud,
	East,
	West,
}

enum MessageScope {
	Global,
	Room,
	Group,
}

impl Command {
	// a voir si je ne met pas ces fonction independantes !!!!
	pub fn parse_command(paquet: String) -> Result<Self, String> {
		to_do();
	}

	pub fn handle_command(command: Self) -> String {
		match command {
			Command::Connect { name } => connect_player(name),
			Command::Look => look(),
			Command::Move {direction} => new_direction(direction),
		}
	}
}