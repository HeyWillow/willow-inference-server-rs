use askama::Template;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LicenseEntry {
    pub authors: Option<String>,
    pub license: String,
    pub name: String,
    pub version: String,
}

impl LicenseEntry {
    #[must_use]
    pub fn authors_list(&self) -> Vec<&str> {
        self.authors
            .as_deref()
            .map(|a| a.split('|').map(str::trim).collect())
            .unwrap_or_default()
    }
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub git_hash: Option<&'static str>,
    pub licenses: Vec<LicenseEntry>,
    pub version: &'static str,
}
