extern crate yaml_rust;
extern crate sks_app_lib;

use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "config.yml";
    let mut f = File::open(filename).expect(&format!("Config file {} not found", filename));

    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to read config file");

    let docs: Vec<Yaml> = YamlLoader::load_from_str(&s).unwrap();

    // We only care about the first document
    let doc: &Yaml = &docs[0];

    let address: String = format!(
        "{}:{}",
        doc["hkp_address"].as_str().unwrap(),
        doc["hkp_port"].as_i64().unwrap()
    );

    sks_app_lib::start(&address);
}
