use clap::ArgMatches;

use std::path::{Path, PathBuf};
use std::fs;

use crate::SETTINGS;

pub fn create_program(args: &ArgMatches) {
    let destination = Path::new(args.get_one::<String>("filename").expect("filename is required"));
    let language = destination.extension().unwrap();

    let mut template = PathBuf::new();  //todo: fix this hot mess
    template.push(&SETTINGS.get().source_generation_settings.templates.as_path());
    template.push("template");
    template.set_extension(&language);

    fs::copy(template, destination).unwrap();

    if let Some(_open_automatically @ true) = args.get_one::<bool>("open") {
        let editor = &SETTINGS.get().source_generation_settings.editor;
        let editor_args = SETTINGS.get().source_generation_settings.editor_args.split(' ').collect::<Vec<_>>();

        std::process::Command::new(editor)
                        .arg(destination)
                        .args(editor_args)
                        .status()
                        .expect("Fatal: could not automatically open the generated file");
    }
}