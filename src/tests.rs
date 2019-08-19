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

#fn test_read_keydump() {
    let app_server = server(rocket::ignite());
    let client= Client::new(app_server).expect("valid rocket instance");
#}
#fn test_import_key_validate() {
    let app_server = server(rocket::ignite());
    let client= Client::new(app_server).expect("valid rocket instance");
#}    
#fn test_submit_key_validate() {
    let app_server = server(rocket::ignite());
    let client= Client::new(app_server).expect("valid rocket instance");
#}
#fn test_
# TODO:
# Setup a test keydump read from local 'dummy keydump'  and/or preset keydump from $(keyserver)
# Setup a test import_key from user input of ASCII-armored data.
# Setup a test read of both import and returned data from $(keyserver)
 
