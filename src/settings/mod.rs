mod source_generation_settings;
mod compiler_settings;
mod test_settings;

use source_generation_settings::SourceGenerationSettings;
use compiler_settings::CompilerSettings;
use test_settings::TestSettings;
use serde::{Serialize, Deserialize};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub source_generation_settings: SourceGenerationSettings,
    pub compiler_settings: CompilerSettings,
    test_settings: TestSettings,
}

impl Settings {
    //R.O.N. stands for Rusty Object Notation: https://github.com/ron-rs/ron
    pub fn from_ron(file: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(file)?;
        let settings: Settings = ron::from_str(&text)?;

        Ok(settings)
    }
}