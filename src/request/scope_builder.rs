#[derive(Default)]
pub struct ScopeBuilder {
    scopes: Vec<String>,
}

impl ScopeBuilder {
    pub fn add_scope(&mut self, scope: &str) {
        self.scopes.push(scope.to_string());
    }

    pub fn add_scopes<'de, I>(&mut self, scopes: I)
    where
        I: IntoIterator<Item = &'de str>,
    {
        self.scopes.extend(
            scopes
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        );
    }
    pub fn build(self) -> String {
        self.scopes.join(" ")
    }
}
