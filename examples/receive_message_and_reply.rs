use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();
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
            println!("got message");
            match &message.reply_to {
              Some(destination) => {
                println!("destination {:?}:{}",destination.destination_type,destination.destination_name);
                let reply_message = TextMessage{
                  header: None,
                  body: "hallo welt".to_string(),
                };
                let _ignore = session.send_message(destination.clone(),reply_message.into());
              },
              None=>{
                println!("no destination found");
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
  }
}
