use serde_json::Value;
// ATTENTION LIBRAIRIE EXTERNE A IMPORTER AVEC CARGO TOML
// on peut utilise json! pour creer un json a partir d'une string
// let json_string = serde_json::to_string(&room_state).unwrap(); pour passer en string pour le paquet

pub enum SuccessMessage {
    Connect,
    Look {data: Value},
    Move {new_room: String},
    Quit,
    Chat,
    Who {players: u16},
    CreateGroup {name: String},
    GroupInvit,
    GroupJoin {name: String},
    GroupLeave,
    Take {item: String},
    Drop {item: String},
    Inventory {data: Value}, // JSON array
    Talk {msg: String},
    Attack {data: Value},
    Status {data: Value},
    Quest {data: Value},
    Quests {data: Value},
    // Help,
}

pub fn success_to_string(success: SuccessMessage) -> Result<String, serde_json::Error> {
    let arg: String = match success {
        SuccessMessage::Connect => "hello proto=1".to_string(),
        
        SuccessMessage::Quit => "bye".to_string(),

        SuccessMessage::Chat
        | SuccessMessage::GroupInvit
        | SuccessMessage::GroupLeave => String::new(),

        SuccessMessage::Look {data}
        | SuccessMessage::Inventory {data}
        | SuccessMessage::Attack {data}
        | SuccessMessage::Status {data}
        | SuccessMessage::Quest {data}
        | SuccessMessage::Quests {data}=> serde_json::to_string(&data)?,

        SuccessMessage::Move {new_room} => format!("room={}", new_room),

        SuccessMessage::Who {players} => format!("players={}", players),

        SuccessMessage::CreateGroup {name}
        | SuccessMessage::GroupJoin {name} => format!("group={}", name),

        SuccessMessage::Take {item} => format!("taken={}", item),

        SuccessMessage::Drop {item} => format!("dropped={}", item),

        SuccessMessage::Talk {msg} => msg,
    };

    if arg.is_empty() {
        Ok("OK".to_string())
    } else {
        Ok(format!("OK {}", arg))
    }
}