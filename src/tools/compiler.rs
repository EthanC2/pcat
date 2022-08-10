use clap::ArgMatches;

use crate::SETTINGS;

pub fn compile_program(args: &ArgMatches) {
    let compiler = &SETTINGS.get().compiler_settings.compiler;
    let filename = args.get_one::<String>("filename").expect("filename is required");
    let executable_name = String::from("-o") + &SETTINGS.get().compiler_settings.executable_name;
    
    let mut compiler_args = &SETTINGS.get().compiler_settings.default_args;
    if let Some(_use_debug @ true) = args.get_one::<bool>("debug") {
        compiler_args = &SETTINGS.get().compiler_settings.debug_args;
    }

    let compiler_args = compiler_args.split(' ').collect::<Vec<_>>();

    std::process::Command::new(&compiler)
                    .arg(filename)
                    .arg(executable_name)
                    .args(compiler_args)
                    .spawn()
                    .expect("Failed to compile");
}