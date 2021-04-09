use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::{TextMessage, BytesMessage, MapMessage};
use tibco_ems::MessageType;
use tibco_ems::GetStringValue;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  {
    let session = connection.session().unwrap();

    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let consumer = session.queue_consumer(destination,None).unwrap();
    
    println!("waiting 10 seconds for a message");
    let msg_result = consumer.receive_message(Some(10000));

    match msg_result {
      Ok(result_value) => {
        match result_value {
          Some(message) => {
            match message.message_type {
              MessageType::TextMessage =>{
                println!("received text message");
                let text_message = TextMessage::from(message);
                println!("header: {:?}",text_message.header);
                //access single header value
                let header = text_message.header.clone().unwrap();
                if header.contains_key("CUSTOM_HEADER_1") {
                  let typed_header = header.get("CUSTOM_HEADER_1").unwrap();
                  let header_value = typed_header.string_value().unwrap();
                  println!("CUSTOM_HEADER_1: {}",header_value);
                }
              },
              MessageType::BytesMessage =>{
                println!("received bytes message");
                let bytes_message = BytesMessage::from(message);
                println!("header: {:?}",bytes_message.header);
              },
              MessageType::MapMessage =>{
                println!("received map message");
                let map_message = MapMessage::from(message);
                println!("header: {:?}",map_message.header);
              },
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
  }
}
