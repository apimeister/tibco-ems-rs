use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string());

  let session = tibco_ems::session(connection);

  let msg = TextMessage{body:"hallo welt".to_string(),header: None};

  let destination = Destination{
    destination_type: DestinationType::Queue,
    destination_name: "myqueue".to_string(),
  };
  tibco_ems::send_text_message(session, destination, msg);

  tibco_ems::session_close(session);
}
