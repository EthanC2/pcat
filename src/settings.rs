use std::env;
use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};

//All members must have statically known sizes for 'state::Storage<T>'
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub program_file: Box<PathBuf>,
    pub settings: Box<PathBuf>,
    pub templates: Box<PathBuf>,
    /*
        OutputExecutableName
    */
}

impl Settings {
    // pub fn new() -> Self {
    //     println!("default initialization!");

    //     let home_directory = env::var("HOME").unwrap();

    //     Self { 
    //         program_file: Box::new(PathBuf::from(&home_directory)),
    //         settings: Box::new(Path::new(&home_directory).join(".pcat/settings.ron"))
    //     }
    // }

    //R.O.N. stands for Rusty Object Notation: https://github.com/ron-rs/ron
    pub fn from_ron(path: &Path) -> Self {
        let home_directory = env::var("HOME").unwrap();

        Self { 
            program_file: Box::new(PathBuf::from(&home_directory)),
            settings: Box::new(Path::new(&home_directory).join(".pcat/settings.ron")),
            templates: Box::new(Path::new(&home_directory).join(".pcat/templates")),
        }
    }
}