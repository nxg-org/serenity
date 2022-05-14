//! Publicly available interaction models.
use serde::{Deserialize};

use crate::model::prelude::ApplicationInfo;
use super::application_command::ApplicationCommand;


#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cursor {
    pub previous: Option<String>,
    pub next: Option<String>,
    pub repaired: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserApplicationCommandSearchResult {
    pub applications: Vec<ApplicationInfo>,
    pub application_commands: Vec<ApplicationCommand>,
    // Too lazy rn.
    // #[serde(skip_deserializing)]
    pub cursor: Cursor
}