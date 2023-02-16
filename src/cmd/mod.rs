use clap::{Arg, ArgAction, Command};
use crate::utils::{VERSION, NAV_VERSION};

mod nav;
pub use nav::run_navigator;

pub enum CmdInput {
    Config(String),
    Navigate,
    None,
}

pub fn scan() -> CmdInput {
    let matches = Command::new("Roma")
        .version(VERSION).long_version("0.6.5 - a2132c")
        .about("A multi-protocol network relay")
        .author("aa51513 <aa51513@github.com>")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("json config file")
                .help("specify a config file in json format")
                .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("nav")
                            .about("An Interactive config editor")
                            .version(NAV_VERSION)
                            .author("aa51513 <aa51513@github.com>"),
        )
        .get_matches();
    if let Some(config) = matches.get_one::<String>("config") {
        return CmdInput::Config(config.to_string());
    }
    if matches.subcommand_matches("nav").is_some() {
        return CmdInput::Navigate;
    }
    CmdInput::None
}
