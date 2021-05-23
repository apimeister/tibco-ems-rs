use tibco_ems::Destination;
use tibco_ems::TextMessage;
use tibco_ems::TypedValue;
use std::collections::HashMap;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  let session = connection.session().unwrap();

  let mut header = HashMap::new();
  header.insert("CUSTOM_HEADER_1".to_string(), TypedValue::String("VALUE_1".to_string()));
  header.insert("CUSTOM_HEADER_2".to_string(), TypedValue::String("VALUE_2".to_string()));
  let msg = TextMessage{
    body:"hallo welt".to_string(),
    header: Some(header),
    ..Default::default()
  };

  let destination = Destination::Queue("myqueue".to_string());

  let _ignore = session.send_message(&destination, msg);
}
