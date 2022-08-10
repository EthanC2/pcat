mod settings;
use settings::Settings;

use state::Storage;
use clap::{Command, Arg, ArgAction, ArgMatches};

use std::path::Path;
use std::env;

static SETTINGS: Storage<Settings> = Storage::new();

fn initialize_settings() {
    let home_directory = env::var("HOME").unwrap();

    if Path::new(&home_directory).join(".pcat/settings.ron").exists() {
        SETTINGS.set(Settings::from_ron());
    } else {
        SETTINGS.set(Settings::new());
    }
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
                        
                        TODO note: can all unexpected flags are passed to the compiler
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

fn main() {
    initialize_settings();
    let args = parse_cmd_args();

    match args.subcommand() {
        Some(("new", suboptions)) => println!("Creating new {} file {}", *suboptions.get_one::<String>("language").expect("language is required"), *suboptions.get_one::<String>("filename").expect("filename is required")),
        Some(("compile", suboptions)) => println!("Compile file {}", *suboptions.get_one::<String>("filename").expect("filename is required")),
        Some(("test", _)) => println!("Testing the executable!"),
        _ => std::unreachable!(),
    }
}
