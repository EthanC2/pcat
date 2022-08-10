use std::env;
use std::path::{Path, PathBuf};

use serde::{Serialize, Deserialize};

//All members must have statically known sizes for 'state::Storage<T>'
#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub storage_path: Box<PathBuf>,
    pub settings_file: Box<PathBuf>,
    /*
        OutputExecutableName
    */
}

impl Settings {
    pub fn new() -> Self {
        println!("default initialization!");

        let home_directory = env::var("HOME").unwrap();

        Self { 
            storage_path: Box::new(PathBuf::from(&home_directory)),
            settings_file: Box::new(Path::new(&home_directory).join(".pcat/settings.ron"))
        }
    }

    //R.O.N. stands for Rusty Object Notation: https://github.com/ron-rs/ron
    pub fn from_ron() -> Self {
        println!("R.O.N. initialization!");

        let home_directory = env::var("HOME").unwrap();

        Self { 
            storage_path: Box::new(PathBuf::from(&home_directory)),
            settings_file: Box::new(Path::new(&home_directory).join(".pcat/settings.ron"))
        }
    }
}