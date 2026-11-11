use serde_json::{Value, json};

// LES CHAT SONT DES EVENTS!!!!!!!!!
#[derive(Debug, PartialEq)]
pub enum Event {
	// ATTENTION TOUTE LA DATA EST EN JSON
	PresenceEnter {scope: EventScope, data: Value}, // ROOM data = {"id": "room", "player": "name"}
	PresenceLeave {scope: EventScope, data: Value}, // ROOM data = {"id": "room", "player": "name"}
	Invite {scope: EventScope, data: Value}, // GROUP data = {"id": "group", "player": name}
	Join {scope: EventScope, data: Value}, // GROUP data = {"id": "group", "player": name}
	Leave {scope: EventScope, data: Value}, //GROUP {"id": "group", "player": name}
	Chat {scope: EventScope, data: Value}, // ROOM//GROUP//GLOBAL// data = {"scope": "global"/"room"/"group", "id": None/"id_group"/"id_room", "player": "name", "message": "plein de mots"}
	Players {scope: EventScope, data: Value}, // data = {"count": int}
}

#[derive(Debug, PartialEq)]
pub enum EventScope {
	Stats,
	Global,
	Room,
	Group,
}

impl Event {
	pub fn info(&self) -> (String, String, String) {
		let (event_type, scope, data) = match self {
			Event::PresenceEnter {scope, data} => ("PRESENCE ENTER", scope, data),
			Event::PresenceLeave {scope, data} => ("PRESENCE LEAVE", scope, data),
			Event::Invite {scope, data} => ("INVITE", scope, data),
			Event::Join {scope, data} => ("JOIN", scope, data),
			Event::Leave {scope, data} => ("LEAVE", scope, data),
			Event::Chat {scope, data} => ("CHAT", scope, data),
			Event::Players {scope, data} => ("", scope, data),
		};

		(event_type.to_string(), 
		scope.to_string(), 
		serde_json::to_string(&data).unwrap())
	}
	// event-line = "EVT" SP event-type SP event-data LF
	fn to_string(&self) -> String {
		let (event_type, scope, data) = self.info();

		format!("EVT {} {} {}", scope, event_type, data)
	}

}

impl EventScope {
	pub fn to_string(&self) -> String {
		match self {
			EventScope::Stats => "STATS".to_string(),
			EventScope::Global => "GLOBAL".to_string(),
			EventScope::Room => "ROOM".to_string(),
			EventScope::Group => "GROUP".to_string(),
		}
	}

}

fn create_data_room(id: String, player: String) -> Value {
	json!({
		"id": id,
		"player": player
	})
}

fn create_data_chat(scope: String, id: String, player: String, message: String) -> Value {
	json!({
		"scope": scope,
		"id": id,
		"player": player,
		"message": message
	})
}

fn create_players_data(count: u16) -> Value {
	json!({
		"count": count
	})
}



// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]

// }

