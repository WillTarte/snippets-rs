use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::DateTime, types::chrono::Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(crate) struct Snippet
{
    id: i32,
    uuid: Uuid,
    name: String,
    description: String,
    content: String,
    owner: String,
    language: snips_lib::Language,
    created_on: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CreateSnippet
{
    name: String,
    description: String,
    content: String,
    language: snips_lib::Language
}