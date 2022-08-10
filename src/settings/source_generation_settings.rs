use serde::{Serialize, Deserialize};

use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct SourceGenerationSettings {
    pub templates: PathBuf,
    pub editor: String,
    pub editor_args: String,
}