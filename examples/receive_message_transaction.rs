use tibco_ems::Destination;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  {
    let session = connection.transacted_session().unwrap();

    let destination = Destination::Queue("myqueue".to_string());
    let consumer = session.queue_consumer(&destination, None).unwrap();
    
    println!("waiting 10 seconds for a message");
    let msg_result = consumer.receive_message(Some(10000));

    match msg_result {
      Ok(result_value) => {
        match result_value {
          Some(message) => {
            // println!("confirming message");
            // message.confirm();
            println!("rolling back message");
            message.rollback();
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
