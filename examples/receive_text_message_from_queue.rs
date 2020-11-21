use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;
use tibco_ems::MessageType;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string());

  let session = tibco_ems::session(connection);

  let destination = Destination{
    destination_type: DestinationType::Queue,
    destination_name: "myqueue".to_string(),
  };
  let consumer = tibco_ems::queue_consumer(session,destination,None);
  
  let msg = tibco_ems::receive_message(consumer, None);

  match msg.message_type {
    MessageType::TextMessage =>{
      println!("received text message");
      let text_message = TextMessage::from(msg);
      println!("content: {}", text_message.body);
    },
    _ => {
      println!("unknown type");
    }
  }
  tibco_ems::session_close(session);
}
