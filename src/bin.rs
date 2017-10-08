extern crate yaml_rust;
extern crate log;

extern crate sks_app_lib;

#[macro_use]
extern crate clap;
use clap::App;

use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Always overrides config file
    let _verbosity = match matches.occurrences_of("v") {
        0 => log::LogLevel::Error,
        1 => log::LogLevel::Info,
        2 => log::LogLevel::Debug,
        3 | _ => log::LogLevel::Trace,
    };

    // Gets a value for config if supplied by user, or defaults to "config.yml"
    let config_file = matches.value_of("config").unwrap_or("config.yml");
    println!("Reading config file {}", config_file);

    let mut f: File = File::open(config_file).expect("Config file not found");

    let mut s: String = String::new();
    f.read_to_string(&mut s).expect(
        "Unable to read config file",
    );

    if let Some(_matches) = matches.subcommand_matches("read_keydump") {
        println!("Reading sks keydumps has not been implemented yet");
        exit(0);
    } else {
        let docs: Vec<Yaml> = YamlLoader::load_from_str(&s).unwrap();

        // We only care about the first document
        let doc: &Yaml = &docs[0];

        let address: &str = &format!(
            "{}:{}",
            doc["hkp_address"].as_str().unwrap(),
            doc["hkp_port"].as_i64().unwrap()
        );

        sks_app_lib::start(&address);
    }
}
