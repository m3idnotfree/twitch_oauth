use super::IRCScopes;

#[derive(Default, Clone)]
pub struct ScopeBuilder {
    scopes: Vec<String>,
}

impl ScopeBuilder {
    /// chat:ediet, chat:read
    pub fn add_irc_scopes(&mut self) {
        self.add_scopes([IRCScopes::ChatEdit, IRCScopes::ChatRead]);
    }

    pub fn add_scope(&mut self, scope: &str) {
        self.scopes.push(scope.to_string());
    }

    pub fn add_scopes<I, T>(&mut self, scopes: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.scopes.extend(
            scopes
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<String>>(),
        );
    }
    pub fn build(self) -> String {
        self.scopes.join(" ")
    }
}
