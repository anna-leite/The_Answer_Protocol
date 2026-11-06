fn connect_player(name: String) -> String {
	println!("{}", name);
	// mecanisme de tcheque du name !!(ERR 201 NAME_IN_USE)
	String::from("OK connected")
}

fn look() -> String {
	// menanisme de la recuperation de la data
	// let answer == mecanisme qui fait construit la string a partir de OK + data en JSON
	String::from("OK data_room_JSON")
}

fn new_direction(direction: Direction) -> String {
	println!("{}", direction);
	// update player data !!(ERR 301 NO_EXIT)
	// si mouvement possible doit renvoyer la nouvelle position
	String::from("OK room=id_room")
}

fn quit() -> String {
	// deconnection propre
	String::from("OK bye")
}

fn send_message(scope: MessageScope, message: String) -> String {
	println!("{} {}", scope, message);
	// verification du scope group
	// ERR 401 NOT_IN_GROUP, ERR 403 GROUP_NOT_FOUND
	// creation du nouveau paquet a envoyer
	// traitement de l'envoi du paquet au scope concerne
	String::from("OK")
}

fn who() -> String {
	// calcul du nb de joueurs dans un salle
	// creation de la reponse a partir de l'int
	String::from("OK players=nb_player")
}

fn create_group(name: String) -> String {
	println!("{}", name);
	// verifie si le nom du groupe est deja utilise
	// ERR 201 NAME_IN_USE
	// creer le group
	String::from("OK group=name")
}

fn group_invit(group_name: &str, player_name: &str) -> String {
	println!("{} {}", group_name, player_name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// verifie si le player qui envoi la demande appartient au group
	// ERR 401 NOT_IN_GROUP
	// verifie si le nom du player exsite
	// ERR 202 NAME_NOT_FOUND
	// verifie si le player appartient deja au groupe
	//  ERR 402 ALREADY_IN_GROUP
	println!("{}", name);
	// ajoute le player au group
	String::from("OK")
}

fn group_join(name: &str) -> String {
	println!("{}", name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// verifie si le player appartient deja au groupe
	//  ERR 402 ALREADY_IN_GROUP
	// ajoute le player au groupe
	String::from("OK group=name")
}

fn group_leave(name: &str) -> String {
	println!("{}", name);
	// verifie l'exitence du nom du group
	// ERR 403 GROUP_NOT_FOUND
	// retire la player du groupe
	String::from("OK")
}

fn take_item(item: &str) -> String {
	println!("{}", item);
	// verifie l'existamce de l'item dans la room qui est la position du player
	// ERR 404 ITEM_NOT_FOUND
	// prend l'item dans son inventory et le retire de la room
	String::from("OK taken=item")
}

fn drop_item(item: &str) -> String {
	println!("{}", item);
	// verifie l'existance de l'item dans l'inventory
	// ERR 404 ITEM_NOT_IN_INVENTORY
	// retire l'item de l'inventory et le replace dans la room 
	String::from("OK dropped=item")
}

fn inventory() -> String {
	// mecanisme de recuperation de la data
	// let answer == mecanisme qui fait construit la string a partir de OK + data en JSON array
	String::from("OK data_inventory_JSON_array")
}

fn talk_ncp(name: &str) -> String {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// macanisme de recuperation de la replique du ncp
	// let answer == construit la reponse a partir de OK et du bout de dialoque
	String::from("OK replique_ncp")
}

fn attack_ncp(name: &str) -> String {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// verifie si le ncp est en mode combat
	// ERR 405 NPC_NOT_HOSTILE
	// mecanisme de recuperation de la data de combat du ncp
	// let answer == construit la reponse a partir de OK et de la data combat ncp en JSON
	String::from("OK data_combat_ncp")
}

fn status_player() -> String {
	// menanisme de recuperation de la data du player 
	// let answer == construit la reponse a partir de OK et de la data du player en JSON
	String::from("OK data_player_JSON")
}

fn ask_for_quest(name: &str) -> String {
	println!("{}", name);
	// verifie la presence du ncp dans la room
	// ERR 404 NPC_NOT_FOUND
	// verifie si le ncp a une quest valide a proposer
	// ERR 406 NO_QUEST_AVAILABLE
	// mecanisme de recuperation de la data de la quest en JSON
	// let answer == construit la reponse a partir de OK et de la data de la quest en JSON
	String::from("OK data_quest_JSON")
}

fn inventory_quests() -> String {
	// menacisme de recuperation de la data de toutes les quests du player en JSON array
	// let answer == construit la reponse a partir de OK et de la data de toues les quests en JSON array
	String::from("OK data_all_quests_JSON_array")
}

fn help() -> String {
	String::from("OK bha debrouille toi!")
}

