use serde_json::Value;
// ATTENTION LIBRAIRIE EXTERNE A IMPORTER AVEC CARGO TOML
// on peut utilise json! pour creer un json a partir d'une string
// let json_string = serde_json::to_string(&room_state).unwrap(); pour passer en string pour le paquet

pub enum Success {
    Connect,
    Look {data: Value},
    Move {new_room: String},
    Quit,
    Chat,
    Who {players: u16},
    GroupCreate {name: String},
    GroupInvit,
    GroupJoin {name: String},
    GroupLeave,
    Take {item: String},
    Drop {item: String},
    Inventory {data: Value}, // JSON array
    Talk {msg: String},
    Attack {data: Value},
    // DEFEND
	// FLEE
	// USE_ITEM
    Status {data: Value},
    Quest {data: Value},
    // ABANDON_QUEST
    // COMPLETE_QUEST
    Quests {data: Value},
    // Help,
}

pub fn success_to_string(success: Success) -> Result<String, serde_json::Error> {
    let arg: String = match success {
        Success::Connect => "hello proto=1".to_string(),
        
        Success::Quit => "bye".to_string(),

        Success::Chat
        | Success::GroupInvit
        | Success::GroupLeave => String::new(),

        Success::Look {data}
        | Success::Inventory {data}
        | Success::Attack {data}
        | Success::Status {data}
        | Success::Quest {data}
        | Success::Quests {data}=> serde_json::to_string(&data)?,

        Success::Move {new_room} => format!("room={}", new_room),

        Success::Who {players} => format!("players={}", players),

        Success::GroupCreate {name}
        | Success::GroupJoin {name} => format!("group={}", name),

        Success::Take {item} => format!("taken={}", item),

        Success::Drop {item} => format!("dropped={}", item),

        Success::Talk {msg} => msg,
    };

    if arg.is_empty() {
        Ok("OK".to_string())
    } else {
        Ok(format!("OK {}", arg))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn you_can_success_to_string() {
//         todo!();
//     }
// }