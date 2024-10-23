use super::Scopes;

/// inspired PathSegmentsMut
/// https://docs.rs/url/latest/src/url/path_segments.rs.html#37-42
#[derive(Debug)]
pub struct ScopesMut<'a> {
    scopes: &'a mut Vec<Scopes>,
}

pub(crate) fn new(scopes: &mut Vec<Scopes>) -> ScopesMut<'_> {
    ScopesMut { scopes }
}

impl<'a> ScopesMut<'a> {
    pub fn push(&mut self, s: Scopes) -> &mut Self {
        self.scopes.push(s);
        self
    }

    pub fn extend<I>(&mut self, scopes: I) -> &mut Self
    where
        I: IntoIterator<Item = Scopes>,
    {
        self.scopes.extend(scopes);
        self
    }

    /// chat:ediet, chat:read
    pub fn add_irc_scopes(&mut self) -> &mut Self {
        self.extend([Scopes::ChatEdit, Scopes::ChatRead]);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::scopes::{self};
    #[test]
    fn scopes_mut() {
        let mut scopes = Vec::new();

        scopes::new(&mut scopes).add_irc_scopes();
        assert_eq!(2, scopes.len());
        assert_eq!(
            "chat:edit chat:read",
            scopes
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
