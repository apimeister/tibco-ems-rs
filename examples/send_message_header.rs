use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;
use std::collections::HashMap;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();
  {
    let session = connection.session().unwrap();

    let mut header = HashMap::new();
    header.insert("CUSTOM_HEADER_1".to_string(), "VALUE_1".to_string());
    header.insert("CUSTOM_HEADER_2".to_string(), "VALUE_2".to_string());
    let msg = TextMessage{body:"hallo welt".to_string(),header: Some(header)};

    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let _ignore = session.send_message(destination, msg.into());
  }
}
