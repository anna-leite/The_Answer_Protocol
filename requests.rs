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

// ERR 901 SEND_FAILED && ERR 900 CONNECTION_FAILED == ATTENTION ERREUR POSSIBLE A CAHQUE ENVOI !!!!!!!!!!!!!!!
// CREER DES NOUVELLE ERREUR POUR LES NOM DE GROUPE PAS TROUVE !!!!!!!!!!!!!!!!!!
//	== ERR 403 GROUP_NOT_FOUND
//	== ERR 202 NAME_NOT_FOUND
//	== ERR 101 INVALID_COMMAND
//	== ERR 102 INVALID_ARGMENTS ???? seulement invalid input????

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
	String::from_utf8(packet)
		.map_err(|_| "ERR 101 INVALID_COMMAND".to_string())
}

fn parse_simple_command(str_packet: &str) -> Option<Command> {
	let mut words = str_packet.split_whitespace();

	match words.next() {
		Some("LOOK") => Some(Command::Look),
		Some("QUIT") => Some(Command::Quit),
		Some("WHO") => Some(Command::Who),
		Some("INVENTORY") => Some(Command::Inventory),
		Some("STATUS") => Some(Command::Status),
		Some("QUESTS") => Some(Command::Quests),
		_ => None,
	}
}

fn parse_move(str_packet: &str) -> Option<Command> {
	let mut words = str_packet.split_whitespace();

	match words.next() {
		Some("MOVE") => {
			match words.next() {
				Some("Nord") => Some(Command::Move {direction: Direction::Nord}),
				Some("Sud") => Some(Command::Move {direction: Direction::Sud}),
				Some("East") => Some(Command::Move {direction: Direction::East}),
				Some("West") => Some(Command::Move {direction: Direction::West}),
				_ => None,
			},
		},
		_ => None,
	}
}

fn parse_string_argument(str_packet: &str) -> Option<Command> {
	let mut words = str_packet.split_whitespace();

	match words.next() {
		Some("TAKE") => Some(Command::Take {item: words.next()?.to_string()}),
		Some("DROP") => Some(Command::Drop {item: words.next()?.to_string()}),
		Some("TALK") => Some(Command::Talk {name: words.next()?.to_string()}),
		Some("ATTACK") => Some(Command::Attack {name: words.next()?.to_string()}),
		Some("QUEST") => Some(Command::Quest {name: words.next()?.to_string()}),
		_ => None,
	}
}

fn parse_group_command(str_packet: &str, count: usize) -> Option<Command> {
	let mut words = str_packet.split_whitespace();

	match (words.next(), words.next()) {
		(Some("CREATE"), Some("GROUP")) if count == 3 => Some("GROUP") => Some(Command::CreateGroup {name: words.next()?.to_string()}),
		(Some("GROUP"), Some("INVITE")) if count == 4 => {
			let group_name = words.next()?.to_string();
			let player_name = words.next()?.to_string();
				
			Some(Command::GroupInvit {group_name, player_name})},
		(Some("GROUP"), Some("JOIN")) if count == 3  => Some(Command::GroupJoin {name: words.next()?.to_string()}),
		(Some("GROUP"), Some("LEAVE")) if count == 3 => Some(Command::GroupLeave {name: words.next()?.to_string()}),
		_ => None,
	}
}

fn parse_chat(str_packet: &str, count: usize) -> Option<Command> {
	let mut words = str_packet.split_whitespace();

	match words.next() {
		Some("CHAT") => {
			match words.next() {
				Some("GLOBAL") => Some(Command::Chat {scope: MessageScope::Global, message: words.collect::<Vec<&str>>().join(" ")}),
				Some("ROOM") => Some(Command::Chat {scope: MessageScope::Room, message: words.collect::<Vec<&str>>().join(" ")}),
				Some("GROUP") if count > 3 => Some(Command::Chat {scope: MessageScope::Group {name: words.next()?.to_string()}, message: words.collect::<Vec<&str>>().join(" ")}),
				_ => None,
			}
		},
		_ => None,
	}
}

fn parse_command(str_packet: String) -> Result<Command, String> {
	// il manque le parsing de help !!!!
	let str_packet = str_packet.strip_suffix("\n").ok_or("ERR 101 INVALID_COMMAND".to_string())?;

	let count = str_packet.split_whitespace().count();

	match count {
		0 => Err("ERR 101 INVALID_COMMAND".to_string()),
		1 => parse_simple_command(str_packet)
			.ok_or_else(|| "ERR 101 INVALID_COMMAND".to_string()),
		2 => parse_move(str_packet)
			.or_else(|| parse_string_argument(&str_packet))
			.ok_or_else(|| "ERR 101 INVALID_COMMAND".to_string()),
		_ => parse_group_command(str_packet, count)
			.or_else(|| parse_chat(str_packet, count))
			.ok_or_else(|| "ERR 101 INVALID_COMMAND".to_string())
	}
}

fn parse_connect(str_packet: String) -> Result<Command, String> {
	let str_packet = str_packet.strip_suffix("\r\n").ok_or("ERR 101 INVALID_COMMAND".to_string())?;

	let count = str_packet.split_whitespace().count();

	match count {
		2 => {
			let mut words = str_packet.split_whitespace();

			match words.next() {
				Some("CONNECT") => Ok(Command::Connect {name: words.next().unwrap().to_string()}),
				_ => Err("ERR 101 INVALID_COMMAND".to_string()),
			}
		}
		_ => Err("ERR 101 INVALID_COMMAND".to_string()),
	}
}

fn parse_packet(packet: Vec<u8) -> Result<Command, String> {
	let str_packet = packet_to_string(packet)?;

	// condition pour verifier si premiere connection 
	// sinon parse_command classique 

	// parse_connect(str_packet)

	parse_command(str_packet)
}

fn handle_command(command: Self) -> String {
	match command {
		Command::Connect {name} => connect_player(name),
		Command::Look => look(),
		Command::Move {direction} => new_direction(direction),
		Command::Quit => quit(),
		Command::Chat {scope, message} => send_message(scope, message), // avoir pour le ownership ou la ref
		Command::Who => who(),
		Command::CreateGroup {name} => create_group(name),
		Command::GroupInvit {group_name, player_name} => group_invit(&group_name, &player_name),
		Command::GroupJoin {name} => group_join(&name),
		Command::GroupLeave {name} => group_leave(&name),
		Command::Take {item} => take_item(&item),
		Command::Drop {item} => drop_item(&item),
		Command::Inventory => inventory(),
		Command::Talk {name} => talk_ncp(&name),
		Command::Attack {name} => attack_ncp(&name),
		// DEFEND
		// FLEE
		// USE_ITEM
		Command::Status => status_player(),
		Command::Quest {name} => ask_for_quest(&name),
		// ABANDON_QUEST
		// COMPLETE_QUEST
		Command::Quests => inventory_quests(),
		Command::Help => help(),
	}
}


// fn encode_answer(answer: &str) -> ?? {}
// ajoute le caractere de fin de message et encode en UTF-8 
// implementer les methodes qui renvoie les str pour faire le paquets en UTF-8

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