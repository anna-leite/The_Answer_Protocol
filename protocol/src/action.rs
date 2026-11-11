use crate::success::Success;
use crate::error::ServerError;
use crate::command::{Direction, MessageScope};
use serde_json::{Value, json};

pub fn connect_player(name: String) -> Result<Success, ServerError> {
	println!("{}", name);
	// validation du format de name
	// mecanisme de tcheque du name !!(ERR 201 NAME_IN_USE)
	Ok(Success::Connect)
}

pub fn look() -> Result<Success, ServerError> {
	// menanisme de la recuperation de la data
	let data: Value = json!({
    "room_id": "forest_01",
    "name": "Dark Forest",
    "description": "A mysterious forest filled with ancient trees."});

	Ok(Success::Look {data})
}

pub fn new_direction(direction: Direction) -> Result<Success, ServerError> {
	let dir_str = match direction {
		Direction::North => "North",
		Direction::Sud => "Sud",
		Direction::East => "East",
		Direction::West => "West",
	};
	println!("{}", dir_str);
	// update player data !!(ERR 301 NO_EXIT)
	// si mouvement possible doit renvoyer la nouvelle position
	let new_room = "grenier".to_string();
	Ok(Success::Move {new_room})
}

pub fn quit() -> Result<Success, ServerError> {
	// deconnection propre
	Ok(Success::Quit)
}

pub fn send_message(scope: MessageScope, message: String) -> Result<Success, ServerError> {
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
	Ok(Success::Chat)
}

pub fn who() -> Result<Success, ServerError> {
	// calcul du nb de joueurs dans un salle
	// creation de la reponse a partir de l'int
	Ok(Success::Who {players: 5})
}

pub fn create_group(name: String) -> Result<Success, ServerError> {
	// println!("{}", name);
	// verifie si le nom du groupe est deja utilise
	// ERR 201 NAME_IN_USE
	// creer le group
	Ok(Success::GroupCreate {name: name.to_string()})
}

pub fn group_invit(group_name: &str, player_name: &str) -> Result<Success, ServerError> {
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
	Ok(Success::GroupInvit)
}

pub fn group_join(name: &str) -> Result<Success, ServerError> {
	// println!("{}", name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// verifie si le player appartient deja au groupe
	//  ERR 402 ALREADY_IN_GROUP
	// ajoute le player au groupe
	Ok(Success::GroupJoin {name: name.to_string()})
}

pub fn group_leave(name: &str) -> Result<Success, ServerError> {
	println!("{}", name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// retire la player du groupe
	Ok(Success::GroupLeave)
}

pub fn take_item(item: &str) -> Result<Success, ServerError> {
	// println!("{}", item);
	// verifie l'existamce de l'item dans la room qui est la position du player
	// ERR 404 ITEM_NOT_FOUND
	// prend l'item dans son inventory et le retire de la room
	Ok(Success::Take {item: item.to_string()})
}

pub fn drop_item(item: &str) -> Result<Success, ServerError> {
	// println!("{}", item);
	// verifie l'existance de l'item dans l'inventory
	// ERR 404 ITEM_NOT_IN_INVENTORY
	// retire l'item de l'inventory et le replace dans la room 
	Ok(Success::Drop {item: item.to_string()})
}

pub fn inventory() -> Result<Success, ServerError> {
	// mecanisme de recuperation de la data
	let data: Value = json!([
        {"id": 1, "name": "Alice", "level": 5},
        {"id": 2, "name": "Bob", "level": 3}
    ]);
	Ok(Success::Inventory {data})
}

pub fn talk_ncp(name: &str) -> Result<Success, ServerError> {
	// println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// macanisme de recuperation de la replique du ncp
	// let answer == construit la reponse a partir de OK et du bout de dialoque
	Ok(Success::Talk {msg:  name.to_string()})
}

pub fn attack_ncp(name: &str) -> Result<Success, ServerError> {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// verifie si le ncp est en mode combat
	// ERR 405 NPC_NOT_HOSTILE
	// mecanisme de recuperation de la data de combat du ncp
	// let answer == construit la reponse a partir de OK et de la data combat ncp en JSON
	let data: Value = json!({
    "exits": {
        "north": "forest_02",
        "south": "village_01",
        "east": null,
        "west": null
    },
    "players": [
        {"id": 1, "name": "Alice", "level": 5},
        {"id": 2, "name": "Bob", "level": 3}
    ]});
	Ok(Success::Attack {data})
}

pub fn status_player() -> Result<Success, ServerError> {
	// menanisme de recuperation de la data du player 
	// let answer == construit la reponse a partir de OK et de la data du player en JSON
	let data: Value = json!({
    "items": ["healing_potion", "wooden_sword"],
    "is_safe": false});
	Ok(Success::Status {data})
}

pub fn ask_for_quest(name: &str) -> Result<Success, ServerError> {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// verifie si le ncp a une quest valide a proposer
	// ERR 406 NO_QUEST_AVAILABLE
	// mecanisme de recuperation de la data de la quest en JSON
	// let answer == construit la reponse a partir de OK et de la data de la quest en JSON
	let data: Value = json!({
    "room_id": "forest_01",
    "name": "Dark Forest",
    "is_safe": false});
	Ok(Success::Quest {data})
}

pub fn inventory_quests() -> Result<Success, ServerError> {
	// menacisme de recuperation de la data de toutes les quests du player en JSON array
	let data: Value = json!({
    "room_id": "forest_01",
    "exits": {
        "north": "forest_02",
        "south": "village_01",
        "east": null,
        "west": null
    },
    "is_safe": false});
	Ok(Success::Quests {data})
}

// pub fn help() -> String {
// 	String::from("OK bha debrouille toi!")
// }

