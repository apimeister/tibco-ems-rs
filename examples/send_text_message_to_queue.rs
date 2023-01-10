use tibco_ems::Destination;
use tibco_ems::TextMessage;

fn main() {
    let url = "tcp://localhost:7222";
    let user = "admin";
    let password = "admin";

    let connection = tibco_ems::connect(url, user, password).unwrap();
    let session = connection.session().unwrap();

    let msg = TextMessage {
        body: "hallo welt".to_string(),
        ..Default::default()
    };

    let destination = Destination::Queue("myqueue".to_string());

    let _ignore = session.send_message(&destination, msg);
}
