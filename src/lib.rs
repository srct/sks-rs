#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)] mod tests;

use std::io::Cursor;
use std::str::FromStr;

use rocket::config::{Config, Environment};
use rocket::request::{Form, FromFormValue};
use rocket::response::Response;
use rocket::http::{Status,RawStr};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn about() -> &'static str {
    "About"
}

#[derive(Debug)]
enum SearchOperation {
    Get,
    Index,
    VerboseIndex
}

impl FromStr for SearchOperation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(SearchOperation::Get),
            "index" => Ok(SearchOperation::Index),
            "vindex" => Ok(SearchOperation::VerboseIndex),
            _ => Err("Invalid operation. Supported operations are 'get', 'index', and 'vindex'.")
        }
    }
}

impl<'v> FromFormValue<'v> for SearchOperation {
    type Error = &'static str;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        SearchOperation::from_str(form_value)
    }
}

#[derive(Debug)]
#[derive(FromForm)]
struct HkpRequestParameters {
    op: SearchOperation,
    search: String,
    mr: Option<String>,
    fingerprint: Option<String>,
    exact: Option<String>
}

#[get("/?<search_parameters>")]
fn pks_lookup(search_parameters: HkpRequestParameters) -> Result<Response<'static>, Status> {
    println!("{:?}", search_parameters);
    match search_parameters.op {
        SearchOperation::Get => Response::build().status(Status::NotImplemented).ok(),
        SearchOperation::Index => Response::build().status(Status::NotImplemented).ok(),
        SearchOperation::VerboseIndex => Response::build().status(Status::NotImplemented).ok(),
    }
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
