use tibco_ems::Destination;
use tibco_ems::BytesMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  let session = connection.session().unwrap();

  let msg = BytesMessage{
    body: vec![1,2,3],
    ..Default::default()
  };

  let destination = Destination::Queue("myqueue".to_string());
  let _ignore = session.send_message(&destination, msg);
}
