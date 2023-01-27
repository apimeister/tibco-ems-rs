use tibco_ems::Destination;
use tibco_ems::Message;
use tibco_ems::TextMessage;

fn main() {
    let url = "tcp://localhost:7222";
    let user = "admin";
    let password = "admin";

    let connection = tibco_ems::connect(url, user, password).unwrap();
    {
        let session = connection.session().unwrap();

        let destination = Destination::Queue("myqueue".to_string());
        let consumer = session.queue_consumer(&destination, None).unwrap();

        println!("waiting 10 seconds for a message");
        let msg_result = consumer.receive_message(Some(10000));

        match msg_result {
            Ok(result_value) => match result_value {
                Some(message) => {
                    println!("got message");
                    match &message {
                        Message::TextMessage(msg) => {
                            match &msg.reply_to {
                                Some(destination) => {
                                    println!("destination {:?}", destination);
                                    let reply_message = TextMessage {
                                        body: "hallo welt".to_string(),
                                        ..Default::default()
                                    };
                                    let _ignore = session.send_message(destination, reply_message);
                                }
                                None => {
                                    println!("no destination found");
                                }
                            };
                        }
                        _ => {}
                    }
                }
                None => {
                    println!("no message returned");
                }
            },
            Err(status) => {
                println!("returned status: {status:?}");
            }
        }
    }
}
