use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::MapMessage;
use tibco_ems::TypedValue;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  {
    let session = connection.session().unwrap();

    let mut msg: MapMessage = Default::default();
    msg.body.insert("boolean_value".to_string(),TypedValue::bool_value(true));
    msg.body.insert("binary_value".to_string(),TypedValue::binary_value(&[1,2,3]));
    msg.body.insert("double_value".to_string(),TypedValue::float_value(1.0));
    msg.body.insert("long_value".to_string(),TypedValue::long_value(i64::MAX));
    msg.body.insert("int_value".to_string(),TypedValue::int_value(i32::MAX));
    msg.body.insert("string_value".to_string(),TypedValue::string_value("hallo welt".to_string()));
    
    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let _ignore = session.send_message(destination, msg.into());
  }
}
