use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum IRCScopes {
    /// Send chat messages to a chatroom using an IRC connection.
    ChatEdit,
    /// View chat messages sent in a chatroom using an IRC connection.
    ChatRead,
}

impl IRCScopes {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChatEdit => "chat:edit",
            Self::ChatRead => "chat:read",
        }
    }
}

impl Display for IRCScopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatEdit => write!(f, "chat:edit"),
            Self::ChatRead => write!(f, "chat:read"),
        }
    }
}

impl From<IRCScopes> for String {
    fn from(val: IRCScopes) -> Self {
        match val {
            IRCScopes::ChatEdit => "chat:edit".to_string(),
            IRCScopes::ChatRead => "chat:read".to_string(),
        }
    }
}
