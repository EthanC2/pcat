mod template_settings;
mod compiler_settings;
mod test_settings;

use template_settings::TemplateSettings;
use compiler_settings::CompilerSettings;
//use test_settings::TestSettings;

use serde::{Serialize, Deserialize};

use std::path::Path;
use std::error::Error;

//All members must have statically known sizes for 'state::Storage<T>', hence the boxing
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub template_settings: TemplateSettings,
    pub compiler_settings: CompilerSettings,
    //test_settings: TestSettings,
}

impl Settings {
    //R.O.N. stands for Rusty Object Notation: https://github.com/ron-rs/ron
    pub fn from_ron(file: &Path) -> Result<Self, Box<dyn Error>> {
        let text = std::fs::read_to_string(file)?;
        let settings: Settings = ron::from_str(&text)?;

        Ok(settings)
    }
}