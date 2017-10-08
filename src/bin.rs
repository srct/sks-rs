extern crate yaml_rust;
extern crate sks_app_lib;

use yaml_rust::{Yaml, YamlLoader};

fn main() {
    let s: &str = "
---
hkp_address: 0.0.0.0
hkp_port: 11371
";
    let docs: Vec<Yaml> = YamlLoader::load_from_str(s).unwrap();

    // We only care about the first document
    let doc: &Yaml = &docs[0];

    let address: String = format!(
        "{}:{}",
        doc["hkp_address"].as_str().unwrap(),
        doc["hkp_port"].as_i64().unwrap()
    );

    sks_app_lib::start(&address);
}
