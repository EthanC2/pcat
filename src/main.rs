mod settings;
use settings::Settings;

mod tools;
use tools::source_generation;
use tools::compiler;
use tools::test;

use state::Storage;
use clap::{Command, Arg, ArgAction, ArgMatches};

use std::path::Path;
use std::env;

static SETTINGS: Storage<Settings> = Storage::new();

fn initialize_settings() {
    let home_directory = env::var("HOME").expect("could not get $HOME environment variable. If you are using Windows, reinstall and run this program in the Windows Subsystem for Linux");
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
                        .arg(
                            Arg::new("open")
                            .long("open")
                            .short('o')
                            .action(ArgAction::SetTrue)
                            .help("Automatically open the created file with $EDITOR")
                        )
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
                        .help("The name of the file to compile")
                        .index(1)
                    )
                    .arg(
                        Arg::new("debug")
                        .long("debug")
                        .short('d')
                        .action(ArgAction::SetTrue)
                        .help("Complies the program with \'debug_args\' instead of \'default_args\'")
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

fn main() {
    initialize_settings();
    let args = parse_cmd_args();

    match args.subcommand() {
        Some(("new", suboptions)) => source_generation::create_program(&suboptions),
        Some(("compile", suboptions)) => compiler::compile_program(&suboptions),
        Some(("test", _)) => test::test_program(),
        _ => std::unreachable!(),
    }
}
