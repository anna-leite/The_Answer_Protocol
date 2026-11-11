use serde_json::Value;

// LES CHAT SONT DES EVENTS!!!!!!!!!
#[derive(Debug, PartialEq)]
pub enum Event {
	PresenceEnter {scope: EventScope, player: String}, // ROOM data = {"room": "id", "player": "name"}
	PresenceLeave {scope: EventScope, player: String}, // ROOM data = {"room": "id", "player": "name"}
	Invite {scope: EventScope, guest_player: String}, // GROUP data = {"group": "name", "player": name}
	Join {scope: EventScope, player: String}, // GROUP data = {"group": "name", "player": name}
	Leave {scope: EventScope, player: String}, //GROUP {"group": "name", "player": name}
	Chat {scope: EventScope, data: Value}, // ROOM//GROUP//GLOBAL// data = {"global": bool, "room": None/"id", "group": None/"name", "player": "name", "message": "plein de mots"} en JSON
	Players {scope: EventScope, player: u16}, // data = int
}

#[derive(Debug, PartialEq)]
pub enum EventScope {
	Stats,
	Global,
	Room {id: String},
	Group {name: String}
}

fn scope_to_string(scope: EventScope) -> String {
	match scope {
		EventScope::Stats => "STATS".to_string(),
		EventScope::Global => "GLOBAL".to_string(),
		EventScope::Room => "ROOM".to_string(),
		EventScope::Group => "GROUP"
	}
}

// fn event_type_to_string(event: Event) -> String {
// 	match event {
// 		Event::PresenceEnter => "PRESENCE ENTER".to_string(),
// 		Event::PresenceLeave => "PRESENCE LEAVE".to_string(),
// 		Event::Invite => "INVITE".to_string(),
// 		Event::Join => "JOIN".to_string(),
// 		Event::Leave => "LEAVE".to_string(),
// 		Event::Chat => "CHAT".to_string(),
// 		Event::Players => String::new(),
// 	}
// }

fn create_data(event: Event) -> Value {
	todo!();
}

fn data_to_string(event: Event) -> String {
	todo!();
}

// ou bien impl info??
fn extract_information_from_event(event: Event) -> (String, String, String) {
	match event {
		Event::PresenceEnter {scope, player} => {
			("PRESENCE ENTER".to_string(),
			scope_to_string(scope),
			json!
			)
		},
		Event::PresenceLeave => "PRESENCE LEAVE".to_string(),
		Event::Invite => "INVITE".to_string(),
		Event::Join => "JOIN".to_string(),
		Event::Leave => "LEAVE".to_string(),
		Event::Chat => "CHAT".to_string(),
		Event::Players => String::new(),
	}

}

// event-line = "EVT" SP event-type SP event-data LF
fn event_to_string(event: Event) -> Result<String, ServerError> {
	todo!();

	// format!("EVT {} {} {}", scope, event_type, data)
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn you_can_create_data(){
	// 	todo!();
	// }

	// #[test]
	// fn you_can_data_to_string() {
	// 	todo!();
	// }

	#[test]
	fn you_can_event_type_to_string() {
		assert_eq!(
			event_type_to_string(Event::PresenceEnter {
				scope: EventScope::Room {id: "grenier".to_string()},
				player: "nouille".to_string()}),
				"PRESENCE ENTER".to_string()
		);
		assert_eq!(
			event_type_to_string(Event::PresenceLeave {
				scope: EventScope::Room {id: "grenier".to_string()},
				player: "nouille".to_string()}),
				"PRESENCE LEAVE".to_string()
		);
		assert_eq!(
			event_type_to_string(Event::Invite {
				scope: EventScope::Group {name: "le_s".to_string()},
				guest_player: "nouille".to_string()}),
				"INVITE".to_string()
		);
		assert_eq!(
			event_type_to_string(Event::Join {
				scope: EventScope::Group {name: "le_s".to_string()},
				player: "nouille".to_string()}),
				"JOIN".to_string()
		);
		assert_eq!(
			event_type_to_string(Event::Chat {
				scope: EventScope::Group {name: "le_s".to_string()},
				data: json!({
					"nom": "Alice",
					"message": "hola"})}),
				"JOIN".to_string()
		);
		assert_eq!(
			event_type_to_string(Event::Join {
				scope: EventScope::Stats,
				player: 10}),
				"".to_string()
		);
	}
}

