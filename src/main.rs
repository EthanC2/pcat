mod settings;
use settings::Settings;

use state::Storage;
use clap::{Command, Arg, ArgAction, ArgMatches};

use std::fs;
use std::env;
use std::path::{Path, PathBuf};

static SETTINGS: Storage<Settings> = Storage::new();

fn initialize_settings() {
    let home_directory = env::var("HOME").expect("could not get $HOME environment variable");
    let settings_file = Path::new(&home_directory).join(".pcat/settings.ron");

    if !settings_file.exists() {
        eprintln!("Fatal: settings file does not exist. Have you installed properly with install.sh/.ps1?");
        std::process::exit(20);
    }

    let settings = Settings::from_ron(&settings_file).unwrap();
    SETTINGS.set(settings);
}

fn parse_cmd_args() -> ArgMatches {
    Command::new("pcat")
                .author("Ethan Cox")
                .version("1.0")
                .about("Programming Competition Automation Toolchain (P.C.A.T)")
                .subcommand_required(true)
                .subcommand(
                    //pcat new <filename>
                    Command::new("new")
                        .aliases(&["n", "touch"])
                        .about("Creates a new <LANGUAGE> file with name <FILENAME>")
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
                        --open (-o): automatically open the file in $EDITOR
                        --line-number (-l): automatically skip to <line_number>
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
    let destination = Path::new(args.get_one::<String>("filename").expect("filename is required"));
    let language = destination.extension().unwrap();

    let mut template = PathBuf::new();  //todo: fix this hot mess
    template.push(&SETTINGS.get().template_settings.templates.as_path());
    template.push("template");
    template.set_extension(&language);

    fs::copy(template, destination).unwrap();
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
