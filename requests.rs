enum Command {
	Connect {name: String}, // CONNECT player_name ==> rep[OK, ERR 201 NAME_IN_USE, ERR 900 CONNECTION FAILED]
	Look, // LOOK ==> rep[OK current room state as JSON]
	Move {direction: Direction}, // MOVE dirrection ==> rep[OK room=new_room, ERR 301 NO_EXIT]
	Quit, // QUIT ==> rep[OK bye]
	Chat {scope: MessageScope, message: String}, // CHAT SCOPE "message" ==> rep[OK, ERR 401 NOT_IN_GROUP, ERR 403 GROUP_NOT_FOUND]
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
	// ATTENTION D'AUTRES COMMANDES A IMPLEMENTER POUR LES COMBATS
	// DEFEND
	// FLEE
	// USE_ITEM
	Quest {name: String}, // QUEST ncp_name ==> rep[OK quest_data_as_json, ERR 404 NPC_NOT_FOUND, ERR 406 NO_QUEST_AVAILABLE]
	Quests, // QUESTS ==> rep[OK list_all_player_quest_data_as_JSON_array]
	// ATTENTION D'AUTRES COMMANDES A IMPLEMENTER POUR LES QUESTS
	// COMPLETE_QUEST
	// ABANDON_QUEST
	Help , // HELP optional_command_name ==> rep[OK rules, ERR 101 COMMAND_NOT_FOUND]

}

// implementer les methodes qui renvoie les str pour faire le paquets en UTF-8

// ERR 901 SEND_FAILED && ERR 900 CONNECTION_FAILED == ATTENTION ERREUR POSSIBLE A CAHQUE ENVOI !!!!!!!!!!!!!!!
// CREER DES NOUVELLE ERREUR POUR LES NOM DE GROUPE PAS TROUVE !!!!!!!!!!!!!!!!!!
//	== ERR 403 GROUP_NOT_FOUND
//	== ERR 202 NAME_NOT_FOUND
//	== ERR 101 INVALID_COMMAND
//	== ERR 102 INVALID_ARGMENTS

enum Direction {
	Nord,
	Sud,
	East,
	West,
}

enum MessageScope {
	Global,
	Room,
	Group {name: String},
}

fn packet_to_string(packet: Vec<u8>) -> Result<String, String> {
	let s_packet: String = String::from_utf8(packet).map_err(|_| "ERR 101 INVALID_COMMAND".to_string())?;

	Ok(s_packet)
}

fn parse_command(packet: String) -> Result<Command, String> {
	// je desencode
	// je retire le caractere de fin de paquet
	// je split au espace ====== PEUT ETRE JE FAIS AUTREMENT !!!!!
	// je regarde si le premier mot match avec une commande 
	// Err() == ERR 101 INVALID_COMMAND
	// je compte le nombre d'arguments fournis et je regarde si c'est regarde si le nombre est valide
	// ERR 102 INVALID_ARGUMENTS
	// je regarde si les arguments sont bien compose des caracteres autorises ou du type demande 
	// ERR 102 INVALID_ARGUMENTS
	// je creer mon objet Command a partir du parsing ainsi effectuer
}

// fn handle_command(command: Self) -> String {
// 	match command {
// 		Command::Connect {name} => connect_player(name),
// 		Command::Look => look(),
// 		Command::Move {direction} => new_direction(direction),
// 		Command::Quit => quit(),
// 		Command::Chat {scope, message} => send_message(scope, message), // avoir pour le ownership ou la ref
// 		Command::Who => who(),
// 		Command::CreateGroup {name} => create_group(name),
// 		Command::GroupInvit {group_name, player_name} => group_invit(&group_name, &player_name),
// 		Command::GroupJoin {name} => group_join(&name),
// 		Command::GroupLeave {name} => group_leave(&name),
// 		Command::Take {item} => take_item(&item),
// 		Command::Drop {item} => drop_item(&item),
// 		Command::Inventory => inventory(),
// 		Command::Talk {name} => talk_ncp(&name),
// 		Command::Attack {name} => attack_ncp(&name),
// 		// DEFEND
// 		// FLEE
// 		// USE_ITEM
// 		Command::Status => status_player(),
// 		Command::Quest {name} => ask_for_quest(&name),
// 		// ABANDON_QUEST
// 		// COMPLETE_QUEST
// 		Command::Quests => inventory_quests(),
// 		Command::Help => help(),
// 	}
// }


// fn encode_answer(answer: &str) -> ?? {}
// ajoute le caractere de fin de message et encode en UTF-8 

// et une autre fonction qui envoi !!


#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn you_can_decoder() {
		let valid_packet: Vec<u8> = vec![0xE4, 0xBD, 0xA0,
    0xE5, 0xA5, 0xBD];
		let invalid_packet: Vec<u8> = vec![0xE4, 0xBD];

		assert_eq!(packet_to_string(valid_packet), Ok("你好".to_string()));
		assert_eq!(packet_to_string(invalid_packet), Err("ERR 101 INVALID_COMMAND".to_string()));
	}
}