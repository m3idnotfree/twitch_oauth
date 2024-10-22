use super::Scopes;

#[derive(Default, Clone)]
pub struct ScopeBuilder {
    scopes: Vec<Scopes>,
}

impl ScopeBuilder {
    /// chat:ediet, chat:read
    pub fn add_irc_scopes(&mut self) {
        self.add_scopes([Scopes::ChatEdit, Scopes::ChatRead]);
    }

    pub fn add_scope(&mut self, scope: Scopes) {
        self.scopes.push(scope);
    }

    pub fn add_scopes<I>(&mut self, scopes: I)
    where
        I: IntoIterator<Item = Scopes>,
    {
        self.scopes.extend(scopes);
    }

    pub fn build(self) -> String {
        self.scopes
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
