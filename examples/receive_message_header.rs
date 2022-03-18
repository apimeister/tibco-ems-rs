use enum_extract::extract;
use tibco_ems::Destination;
use tibco_ems::Message;
use tibco_ems::TypedValue;

fn main() {
  let url = "tcp://localhost:7222";
  let user = "admin";
  let password = "admin";

  let connection = tibco_ems::connect(url, user, password).unwrap();
  let session = connection.session().unwrap();

  let destination = Destination::Queue("myqueue".to_string());
  let consumer = session.queue_consumer(&destination, None).unwrap();

  println!("waiting 10 seconds for a message");
  let msg_result = consumer.receive_message(Some(10000));

  match msg_result {
    Ok(result_value) => {
      match result_value {
        Some(message) => {
          match &message {
            Message::TextMessage(text_message) => {
              println!("received text message");
              println!("header: {:?}", text_message.header);
              //access single header value
              let header = text_message.header.clone().unwrap();
              if header.contains_key("CUSTOM_HEADER_1") {
                let typed_header = header.get("CUSTOM_HEADER_1").unwrap();
                let header_value =
                  extract!(TypedValue::String(_), typed_header).expect("extract header");
                println!("CUSTOM_HEADER_1: {}", header_value);
              }
            }
            Message::BytesMessage(bytes_message) => {
              println!("received bytes message");
              println!("header: {:?}", bytes_message.header);
            }
            Message::ObjectMessage(obj_message) => {
              println!("received object message");
              println!("header: {:?}", obj_message.header);
            }
            Message::MapMessage(map_message) => {
              println!("received map message");
              println!("header: {:?}", map_message.header);
            }
          }
        }
        None => {
          println!("no message returned");
        }
      }
    }
    Err(status) => {
      println!("returned status: {:?}", status);
    }
  }
}
