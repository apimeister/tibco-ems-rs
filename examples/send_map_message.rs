use tibco_ems::Destination;
use tibco_ems::MapMessage;
use tibco_ems::TypedValue;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  let session = connection.session().unwrap();

  let mut msg: MapMessage = Default::default();
  msg.body.insert("boolean_value".to_string(),TypedValue::Boolean(true));
  msg.body.insert("binary_value".to_string(),TypedValue::Binary([1,2,3].to_vec()));
  msg.body.insert("double_value".to_string(),TypedValue::Float(1.0));
  msg.body.insert("long_value".to_string(),TypedValue::Long(i64::MAX));
  msg.body.insert("int_value".to_string(),TypedValue::Integer(i32::MAX));
  msg.body.insert("string_value".to_string(),TypedValue::String("hallo welt".to_string()));
  
  let destination = Destination::Queue("myqueue".to_string());

  let _ignore = session.send_message(&destination, msg);
}
