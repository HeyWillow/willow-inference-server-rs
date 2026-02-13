use askama::Template;
use axum::response::Html;

use crate::built_info;
use crate::ui::templates::{AboutTemplate, LicenseEntry};

/// # Panics
/// * when embeddeded licenses.json cannot be deserialized
pub async fn about() -> Html<String> {
    let licenses_str = include_str!(concat!(env!("OUT_DIR"), "/licenses.json"));
    let licenses: Vec<LicenseEntry> = serde_json::from_str(licenses_str)
        .expect("failed to deserialize embedded licenses.json file");

    let template = AboutTemplate {
        git_hash: built_info::GIT_COMMIT_HASH_SHORT,
        licenses,
        version: built_info::PKG_VERSION,
    };

    Html(template.render().unwrap_or_default())
}
