#[cfg(test)]
#[cfg(feature = "test_with_ems")]
mod local_ems {
    use std::collections::HashMap;
    use tibco_ems::Destination;
    use tibco_ems::TextMessage;
    use tibco_ems::TypedValue;
    use tibco_ems::Message;

  #[test]
  fn test_correlation_id_setter() {
    let url = "tcp://localhost:7222";
    let user = "admin";
    let password = "admin";

    let connection = tibco_ems::connect(url, user, password).unwrap();
    let session = connection.session().unwrap();

    let mut header = HashMap::new();
    header.insert(
        "CorrelationID".to_string(),
        TypedValue::String("VALUE_1".to_string()),
    );
    let msg = TextMessage {
        body: "hallo welt".to_string(),
        header: Some(header),
        ..Default::default()
    };

    let destination = Destination::Queue("myqueue".to_string());

    let _ignore = session.send_message(&destination, msg);

    let consumer = session.queue_consumer(&destination, None).unwrap();
    let msg_result = consumer.receive_message(Some(10000));
    match msg_result {
        Ok(res) => {
            match res {
                Some(response_msg) => {
                    match &response_msg {
                        Message::TextMessage(txt_msg) => {
                            println!("got message: {:?}", txt_msg);
                        },
                        _ => {
                            panic!("expected text message");
                        }
                    }
                },
                None =>{
                    panic!("no message received");
                }
            }   
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
  }

  #[test]
  fn test_correlation_id_none() {
      env_logger::init();
    let url = "tcp://localhost:7222";
    let user = "admin";
    let password = "admin";

    let connection = tibco_ems::connect(url, user, password).unwrap();
    let session = connection.session().unwrap();

    let mut header = HashMap::new();
    let msg = TextMessage {
        body: "hallo welt".to_string(),
        header: Some(header),
        ..Default::default()
    };

    let destination = Destination::Queue("myqueue".to_string());

    let _ignore = session.send_message(&destination, msg);

    let consumer = session.queue_consumer(&destination, None).unwrap();
    let msg_result = consumer.receive_message(Some(10000));
    match msg_result {
        Ok(res) => {
            match res {
                Some(response_msg) => {
                    match &response_msg {
                        Message::TextMessage(txt_msg) => {
                            println!("got message: {:?}", txt_msg);
                        },
                        _ => {
                            panic!("expected text message");
                        }
                    }
                },
                None =>{
                    panic!("no message received");
                }
            }   
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
  }
}