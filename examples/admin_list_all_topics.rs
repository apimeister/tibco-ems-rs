fn main() {
    let url = "tcp://localhost:7222";
    let user = "admin";
    let password = "admin";

    let connection = tibco_ems::admin::connect(url, user, password).unwrap();
    let session = connection.session().unwrap();

    let result = tibco_ems::admin::list_all_topics(&session);
    println!("{:?}", result);
}
