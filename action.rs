use crate::models::{MessageScope, Direction};

pub fn connect_player(name: String) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// mecanisme de tcheque du name !!(ERR 201 NAME_IN_USE)
	String::from("OK connected")
}

pub fn look() -> Result<SuccessMessage, CommandError> {
	// menanisme de la recuperation de la data
	// let answer == mecanisme qui fait construit la string a partir de OK + data en JSON
	String::from("OK data_room_JSON")
}

pub fn new_direction(direction: Direction) -> Result<SuccessMessage, CommandError> {
	let dir_str = match direction {
		Direction::Nord => "Nord",
		Direction::Sud => "Sud",
		Direction::East => "East",
		Direction::West => "West",
	};
	println!("{}", dir_str);
	// update player data !!(ERR 301 NO_EXIT)
	// si mouvement possible doit renvoyer la nouvelle position
	String::from("OK room=id_room")
}

pub fn quit() -> Result<SuccessMessage, CommandError> {
	// deconnection propre
	String::from("OK bye")
}

pub fn send_message(scope: MessageScope, message: String) -> Result<SuccessMessage, CommandError> {
	let scope_str = match scope {
		MessageScope::Global => "Global".to_string(),
		MessageScope::Room => "Room".to_string(),
		MessageScope::Group {name} => name,
	};
	println!("{} {}", scope_str, message);
	// verification du scope group
	// ERR 401 NOT_IN_GROUP, ERR 403 GROUP_NOT_FOUND
	// creation du nouveau paquet a envoyer
	// traitement de l'envoi du paquet au scope concerne
	String::from("OK")
}

pub fn who() -> Result<SuccessMessage, CommandError> {
	// calcul du nb de joueurs dans un salle
	// creation de la reponse a partir de l'int
	String::from("OK players=nb_player")
}

pub fn create_group(name: String) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// verifie si le nom du groupe est deja utilise
	// ERR 201 NAME_IN_USE
	// creer le group
	String::from("OK group=name")
}

pub fn group_invit(group_name: &str, player_name: &str) -> Result<SuccessMessage, CommandError> {
	println!("{} {}", group_name, player_name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// verifie si le player qui envoi la demande appartient au group
	// ERR 401 NOT_IN_GROUP
	// verifie si le nom du player exsite
	// ERR 202 NAME_NOT_FOUND
	// verifie si le player appartient deja au groupe
	//  ERR 402 ALREADY_IN_GROUP
	// ajoute le player au group
	String::from("OK")
}

pub fn group_join(name: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// verifie si le player appartient deja au groupe
	//  ERR 402 ALREADY_IN_GROUP
	// ajoute le player au groupe
	String::from("OK group=name")
}

pub fn group_leave(name: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// retire la player du groupe
	String::from("OK")
}

pub fn take_item(item: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", item);
	// verifie l'existamce de l'item dans la room qui est la position du player
	// ERR 404 ITEM_NOT_FOUND
	// prend l'item dans son inventory et le retire de la room
	String::from("OK taken=item")
}

pub fn drop_item(item: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", item);
	// verifie l'existance de l'item dans l'inventory
	// ERR 404 ITEM_NOT_IN_INVENTORY
	// retire l'item de l'inventory et le replace dans la room 
	String::from("OK dropped=item")
}

pub fn inventory() -> Result<SuccessMessage, CommandError> {
	// mecanisme de recuperation de la data
	// let answer == mecanisme qui fait construit la string a partir de OK + data en JSON array
	String::from("OK data_inventory_JSON_array")
}

pub fn talk_ncp(name: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// macanisme de recuperation de la replique du ncp
	// let answer == construit la reponse a partir de OK et du bout de dialoque
	String::from("OK replique_ncp")
}

pub fn attack_ncp(name: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// verifie si le ncp est en mode combat
	// ERR 405 NPC_NOT_HOSTILE
	// mecanisme de recuperation de la data de combat du ncp
	// let answer == construit la reponse a partir de OK et de la data combat ncp en JSON
	String::from("OK data_combat_ncp")
}

pub fn status_player() -> Result<SuccessMessage, CommandError> {
	// menanisme de recuperation de la data du player 
	// let answer == construit la reponse a partir de OK et de la data du player en JSON
	String::from("OK data_player_JSON")
}

pub fn ask_for_quest(name: &str) -> Result<SuccessMessage, CommandError> {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// verifie si le ncp a une quest valide a proposer
	// ERR 406 NO_QUEST_AVAILABLE
	// mecanisme de recuperation de la data de la quest en JSON
	// let answer == construit la reponse a partir de OK et de la data de la quest en JSON
	String::from("OK data_quest_JSON")
}

pub fn inventory_quests() -> Result<SuccessMessage, CommandError> {
	// menacisme de recuperation de la data de toutes les quests du player en JSON array
	// let answer == construit la reponse a partir de OK et de la data de toues les quests en JSON array
	String::from("OK data_all_quests_JSON_array")
}

// pub fn help() -> String {
// 	String::from("OK bha debrouille toi!")
// }

