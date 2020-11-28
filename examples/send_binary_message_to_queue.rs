use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::BytesMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();
  {
    let session = connection.session().unwrap();

    let msg = BytesMessage{body:vec![1,2,3],header: None};

    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let _ignore = session.send_message(destination, msg.into());
  }
}
