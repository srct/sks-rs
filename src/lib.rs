#![feature(try_from)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::convert::TryFrom;

use rocket::config::{Config, Environment};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



pub fn start(address: &str, port: i64) {
    println!("Preparing to listen on http://{}", address);

    let port: u16 = u16::try_from(port).unwrap();
    let config = Config::build(Environment::Staging)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();
    let app = rocket::custom(config, false);
    app.mount("/", routes![index]).launch();

    println!("Listening on http://{}:{}", address, port);
}
