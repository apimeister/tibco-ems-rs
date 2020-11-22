use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();

  let session = tibco_ems::session(connection).unwrap();

  let msg = TextMessage{body:"hallo welt".to_string(),header: None};

  let destination = Destination{
    destination_type: DestinationType::Queue,
    destination_name: "myqueue".to_string(),
  };
  let _ignore = tibco_ems::send_message(session, destination, msg.into());

  tibco_ems::session_close(session);
}
