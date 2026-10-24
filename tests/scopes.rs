use std::collections::BTreeSet;

use scraper::{Html, Selector};
use twitch_oauth_token::Scope;

const SCOPES_URL: &str = "https://dev.twitch.tv/docs/authentication/scopes/";

#[tokio::test]
#[ignore]
async fn matches_twitch_docs() {
    let html = reqwest::get(SCOPES_URL)
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .await
        .unwrap();

    let doc = Html::parse_document(&html);
    let cell = Selector::parse("td > code.highlighter-rouge").unwrap();
    let docs: BTreeSet<&str> = doc
        .select(&cell)
        .filter(|code| code.prev_sibling().is_none() && code.next_sibling().is_none())
        .filter_map(|code| code.text().next())
        .collect();

    assert!(
        !docs.is_empty(),
        "no scopes found, the page layout may have changed"
    );

    let ours: BTreeSet<&str> = Scope::ALL.iter().map(Scope::as_str).collect();
    let missing: Vec<_> = docs.difference(&ours).collect();
    let removed: Vec<_> = ours.difference(&docs).collect();

    assert!(
        missing.is_empty() && removed.is_empty(),
        "missing: {missing:?}\nremoved: {removed:?}"
    );
}
