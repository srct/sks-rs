#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)] mod tests;

use std::io::Cursor;

use rocket::config::{Config, Environment};
use rocket::request::Form;
use rocket::response::Response;
use rocket::http::Status;


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

#[derive(FromForm)]
struct AsciiArmoredKey {
    keytext: String,
}

#[post("/", data = "<form>")]
fn pks_add(form: Form<AsciiArmoredKey>) -> Response {
    let keytext = &form.get().keytext;
    if keytext.starts_with("-----BEGIN PGP PUBLIC KEY BLOCK-----") {
        Response::build().status(Status::NotImplemented).finalize()
    } else {
        Response::build()
            .status(Status::BadRequest)
            .sized_body(Cursor::new("You must submit an ASCII Armored PGP Key"))
            .finalize()
    }
}

fn server(app: rocket::Rocket) -> rocket::Rocket {
    app.mount("/", routes![index])
        .mount("/about", routes![about])
        .mount("/pks/lookup", routes![pks_lookup])
        .mount("/pks/add", routes![pks_add])
}

pub fn start(address: &str, port: u16) {
    println!("Preparing to listen on http://{}:{}/", address, port);

    let config = Config::build(Environment::Staging)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();
    let app = rocket::custom(config, true);

    server(app).launch();
}
