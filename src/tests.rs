#[cfg(test)]
use super::server;
use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn test_hello() {
    let app_server = server(rocket::ignite());
    let client = Client::new(app_server).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
