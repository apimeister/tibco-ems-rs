use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;
use tibco_ems::MessageType;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();

  let session = tibco_ems::session(connection).unwrap();

  let destination = Destination{
    destination_type: DestinationType::Queue,
    destination_name: "myqueue".to_string(),
  };
  let consumer = tibco_ems::queue_consumer(session,destination,None).unwrap();
  
  println!("waiting 10 seconds for a message");
  let msg_result = tibco_ems::receive_message(consumer, Some(10000));

  match msg_result {
    Ok(result_value) => {
      match result_value {
        Some(message) => {
          match message.message_type {
            MessageType::TextMessage =>{
              println!("received text message");
              let text_message = TextMessage::from(message);
              println!("content: {}", text_message.body);
            },
            _ => {
              println!("unknown type");
            }
          }    
        },
        None =>{
          println!("no message returned");
        },
      }
    },
    Err(status) => {
      println!("returned status: {:?}",status);
    }
  }
  tibco_ems::session_close(session);
}
