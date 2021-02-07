use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::MapMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  {
    let session = connection.session().unwrap();

    let mut msg: MapMessage = Default::default();
    msg.body_bool.insert("boolean_value".to_string(),true);
    msg.body_bytes.insert("binary_value".to_string(),vec![1,2,3]);
    msg.body_double.insert("double_value".to_string(),1.0);
    msg.body_long.insert("long_value".to_string(),i64::MAX);
    msg.body_int.insert("int_value".to_string(),i32::MAX);
    msg.body_string.insert("string_value".to_string(),"hallo welt".to_string());
    
    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let _ignore = session.send_message(destination, msg.into());
  }
}
