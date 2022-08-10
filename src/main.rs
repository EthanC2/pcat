mod settings;
use settings::Settings;

use state::Storage;
use clap::{Command, Arg, ArgAction, ArgMatches};

use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

static SETTINGS: Storage<Settings> = Storage::new();

fn initialize_settings() {
    let home_directory = env::var("HOME").expect("could not get $HOME environment variable");
    let local_settings = Path::new(&home_directory).join(".pcat/settings.ron");

    if !local_settings.exists() {
        eprintln!("Fatal: settings file does not exist. Have you installed properly with install.sh/.ps1?");
        std::process::exit(20);
    }

    SETTINGS.set(Settings::from_ron(&local_settings));
}

fn parse_cmd_args() -> ArgMatches {
    Command::new("pcat")
                .author("Ethan Cox")
                .version("1.0")
                .about("Programming Competition Automation Toolchain (P.C.A.T)")
                .subcommand_required(true)
                .subcommand(
                    //pcat new <programming_language> <filename>
                    Command::new("new")
                        .aliases(&["n", "touch"])
                        .about("Creates a new <LANGUAGE> file with name <FILENAME>")
                        .arg(
                            Arg::new("language")
                            .required(true)
                            .takes_value(true)
                            .value_name("LANGUAGE")
                            .action(ArgAction::Set)
                            .help("the language of the file to create")
                            .index(1)
                        )
                        .arg(
                            Arg::new("filename")
                            .required(true)
                            .takes_value(true)
                            .value_name("FILE")
                            .action(ArgAction::Set)
                            .help("the name of the file to compile")
                            .index(2)
                        )
        
                    /*
                    
                    */
                )
                .subcommand(
                    //pcat compile <filename>
                    Command::new("compile")
                    .aliases(&["comp", "c", "build"])
                    .about("Compiles <FILE> using settings from $HOME/.pcat/settings.ron")
                    .arg(
                        Arg::new("filename")
                        .required(true)
                        .takes_value(true)
                        .value_name("FILE")
                        .action(ArgAction::Set)
                        .help("the name of the file to compile")
                        .index(1)
                    )
                    /*
                        debug: compile with debug flags
                        passthrough: passes vector of arguments to the compiler
                        
                        TODO note: can all unexpected flags are passed to the compiler?
                    */
                )
                .subcommand(
                    //pcat test
                    Command::new("test")
                    .aliases(&["t", "debug"])
                    .about("Runs unit tests on the executable")
                    /*
                        details: provide performance details
                    */
                )
                .get_matches()
}

fn create_program(args: &ArgMatches) {
    println!("creating the program");
    let filename = args.get_one::<String>("filename").expect("filename is required");
    let file_extension = args.get_one::<String>("language").expect("language is required");
    
    let templates_folder = &SETTINGS.get().templates;
    let mut template_file = PathBuf::from("template");
    template_file.set_extension(file_extension);

    let mut template = PathBuf::new();
    template.push(&SETTINGS.get().templates.as_path());
    template.push(template_file);

    fs::copy(template, filename).unwrap();
}

fn compile_program(args: &ArgMatches) {
    println!("compiling!");
}

fn test_program() {
    println!("testing program!");
}

fn main() {
    initialize_settings();
    let args = parse_cmd_args();

    match args.subcommand() {
        Some(("new", suboptions)) => create_program(&suboptions),
        Some(("compile", suboptions)) => compile_program(&suboptions),
        Some(("test", _)) => test_program(),
        _ => std::unreachable!(),
    }
}
