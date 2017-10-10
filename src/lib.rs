#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::config::{Config, Environment};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn about() -> &'static str {
    "About"
}

#[get("/")]
fn pks_lookup() -> &'static str {
    "pks_lookup"
}

#[post("/")]
fn pks_add() -> &'static str {
    "pks_add"
}


pub fn start(address: &str, port: u16) {
    println!("Preparing to listen on http://{}", address);

    let config = Config::build(Environment::Staging)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();
    let app = rocket::custom(config, false);

    app.mount("/", routes![index])
        .mount("/about", routes![about])
        .mount("/pks/lookup", routes![pks_lookup])
        .mount("/pks/add", routes![pks_add])
        .launch();

    println!("Listening on http://{}:{}", address, port);
}
