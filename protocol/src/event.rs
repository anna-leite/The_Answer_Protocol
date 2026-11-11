use serde_json::Value;

// LES CHAT SONT DES EVENTS!!!!!!!!!
#[derive(Debug, PartialEq)]
pub enum Event {
	// ATTENTION TOUTE LA DATA EST EN JSON
	PresenceEnter {scope: EventScope, data: Value}, // ROOM data = {"room": "id", "player": "name"}
	PresenceLeave {scope: EventScope, data: Value}, // ROOM data = {"room": "id", "player": "name"}
	Invite {scope: EventScope, data: Value}, // GROUP data = {"group": "name", "player": name}
	Join {scope: EventScope, data: Value}, // GROUP data = {"group": "name", "player": name}
	Leave {scope: EventScope, data: Value}, //GROUP {"group": "name", "player": name}
	Chat {scope: EventScope, data: Value}, // ROOM//GROUP//GLOBAL// data = {"global": bool, "room": None/"id", "group": None/"name", "player": "name", "message": "plein de mots"}
	Players {scope: EventScope, data: Value}, // data = {"count": int}
}

#[derive(Debug, PartialEq)]
pub enum EventScope {
	Stats,
	Global,
	Room,
	Group,
}

fn scope_to_string(scope: EventScope) -> String {
	match scope {
		EventScope::Stats => "STATS".to_string(),
		EventScope::Global => "GLOBAL".to_string(),
		EventScope::Room => "ROOM".to_string(),
		EventScope::Group => "GROUP".to_string(),
	}
}
// ou bien impl info??
fn extract_information_from_event(event: Event) -> (String, String, String) {
	let (event_type, scope, data) = match event {
		Event::PresenceEnter {scope, data} => ("PRESENCE ENTER", scope, data),
		Event::PresenceLeave {scope, data} => ("PRESENCE LEAVE", scope, data),
		Event::Invite {scope, data} => ("INVITE", scope, data),
		Event::Join {scope, data} => ("JOIN", scope, data),
		Event::Leave {scope, data} => ("LEAVE", scope, data),
		Event::Chat {scope, data} => ("CHAT", scope, data),
		Event::Players {scope, data} => ("", scope, data),
	}

	(event_type.to_string(), 
	scope_to_string(scope), 
	serde_json::to_string(&data).unwrap())
}

// event-line = "EVT" SP event-type SP event-data LF
fn event_to_string(event: Event) -> Result<String, ServerError> {
	let (event_type, scope, data) = extract_information_from_event(event);

	format!("EVT {} {} {}", scope, event_type, data)
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

