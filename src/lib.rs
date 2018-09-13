#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

#[cfg(test)]
mod tests;
use std::io::Cursor;
use std::str::FromStr;

use std::collections::HashMap;

use rocket::config::{Config, Environment};
use rocket::request::{Request, Form, FromFormValue};
use rocket::response::Response;
use rocket::http::{Status, RawStr};
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let mut map = HashMap::new();
    map.insert("name", "<strong>gmu</strong>keyserver");
    Template::render("index", &map)
}

#[get("/")]
fn about() -> Template {
    let map: HashMap<&str, &str> = HashMap::new();
    Template::render("about", &map)
}

#[derive(Debug)]
enum SearchOperation {
    Get,
    Index,
    VerboseIndex,
}

impl FromStr for SearchOperation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(SearchOperation::Get),
            "index" => Ok(SearchOperation::Index),
            "vindex" => Ok(SearchOperation::VerboseIndex),
            _ => Err(
                "Invalid operation. Supported operations are 'get', 'index', and 'vindex'.",
            ),
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
    op: Result<SearchOperation, &'static str>,
    search: String,
    mr: Option<String>,
    fingerprint: Option<String>,
    exact: Option<String>,
}

#[get("/?<search_parameters>")]
fn pks_lookup(search_parameters: HkpRequestParameters) -> Result<Response<'static>, Status> {
    println!("{:?}", search_parameters);
    match search_parameters.op {
        Ok(SearchOperation::Get) => Response::build().status(Status::NotImplemented).ok(),
        Ok(SearchOperation::Index) => Response::build().status(Status::NotImplemented).ok(),
        Ok(SearchOperation::VerboseIndex) => Response::build().status(Status::NotImplemented).ok(),
        Err(error) => {
            Response::build()
                .status(Status::UnprocessableEntity)
                .sized_body(Cursor::new(error))
                .ok()
        }
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

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut ctx = HashMap::new();
    ctx.insert("path", req.uri().as_str());
    Template::render("error/404", &ctx)
}

fn server(app: rocket::Rocket) -> rocket::Rocket {
    app.mount("/", routes![index])
        .mount("/about", routes![about])
        .mount("/pks/lookup", routes![pks_lookup])
        .mount("/pks/add", routes![pks_add])
        .attach(Template::fairing())
        .catch(catchers![not_found])
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
