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

fn server(app: rocket::Rocket) -> rocket::Rocket {
    app.mount("/", routes![index])
        .mount("/about", routes![about])
        .mount("/pks/lookup", routes![pks_lookup])
        .mount("/pks/add", routes![pks_add])
}

pub fn start(address: &str, port: u16) {
    println!("Preparing to listen on http://{}", address);

    let config = Config::build(Environment::Staging)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();
    let app = rocket::custom(config, false);

    server(app).launch();

    println!("Listening on http://{}:{}", address, port);
}

#[cfg(test)]
mod test {
    extern crate rocket;
    use super::server;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_hello() {
        let app_server = server(rocket::ignite());
        let client = Client::new(app_server).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
