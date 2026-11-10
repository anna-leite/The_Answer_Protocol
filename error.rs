#[repr(u16)]
pub enum CommandError {
    InvalidCommand,
    // InvalidArgument,
    // NameInUse,
    // NoExit,
    // NotInGroup,
    // AlreadyInGroup,
    // GroupNotFound,
    // PlayerNotFound,
    // ItemNotFound,
    // ItemNotInInventory,
    // NcpNotFound,
    // NcpNotHostile,
    // NoQuestAvailable,
    // ConnectionFailed,
    // SendFailed,
}

impl CommandError {
    pub fn info(&self) -> (u16, &str) {
        match self {
            Self::InvalidCommand => (101, "INVALID_COMMAND"),
            // Self::InvalidArgument => (101, "INVALID_ARGMENT"),
            // Self::NameInUse => (201, "NAME_IN_USE"),
            // Self::NoExit => (301, "NO_EXIT"),
            // Self::NotInGroup => (401, "NOT_IN_GROUP"),
            // Self::AlreadyInGroup => (402, "ALREADY_IN_GROUP"),
            // Self::GroupNotFound => (404, "GROUP_NOT_FOUND"),
            // Self::PlayerNotFound => (404, "PLAYER_NOT_FOUND"),
            // Self::ItemNotFound => (404, "ITEM_NOT_FOUND"),
            // Self::ItemNotInInventory => (404, "ITEM_NOT_IN_INVENTORY"),
            // Self::NcpNotFound => (404, "NPC_NOT_FOUND"),
            // Self::NcpNotHostile => (405, "NPC_NOT_HOSTILE"),
            // Self::NoQuestAvailable => (406, "NO_QUEST_AVAILABLE"),
            // Self::ConnectionFailed => (900, "CONNECTION_FAILED"),
            // Self::SendFailed => (901, "SEND_FAILED"), 
        }
    }
}

pub fn error_to_string(error: CommandError) -> String {
    let (id, message) = error.info();

    format!("ERR {} {}", id, message)
}

// ERR 901 SEND_FAILED && ERR 900 CONNECTION_FAILED == ATTENTION ERREUR POSSIBLE A CAHQUE ENVOI !!!!!!!!!!!!!!!
// CREER DES NOUVELLE ERREUR POUR LES NOM DE GROUPE PAS TROUVE !!!!!!!!!!!!!!!!!!
//	== ERR 404 GROUP_NOT_FOUND
//	== ERR 404 PLAYER_NOT_FOUND
//	== ERR 101 INVALID_COMMAND
//	== ERR 101 INVALID_ARGMENTS == peut etre a utiliser pour la composition alpha numerique