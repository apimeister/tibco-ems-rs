fn main() {
    let url = "tcp://localhost:7222";
    let user = "admin";
    let password = "admin";

    let connection = tibco_ems::admin::connect(url, user, password).unwrap();
    let session = connection.session().unwrap();

    let state = tibco_ems::admin::get_server_state(&session);
    println!("server state: {:?}", state);
}
