#[cfg(test)]
#[cfg(not(feature = "ems-sys"))]
mod mock {

    #[test]
    fn send_text_message() -> Result<(), String> {
        let conn = tibco_ems::connect("tcp://example.org:7222", "admin", "admin").unwrap();
        let session = conn.session().unwrap();
        let queue = tibco_ems::Destination::Queue("test.queue".to_string());
        let msg = tibco_ems::TextMessage {
            body: "Hello World!".to_string(),
            ..Default::default()
        };
        session.send_message(&queue, msg).unwrap();

        Ok(())
    }

    #[test]
    fn receive_text_message() -> Result<(), String> {
        let conn = tibco_ems::connect("tcp://example.org:7222", "admin", "admin").unwrap();
        let session = conn.session().unwrap();
        let queue = tibco_ems::Destination::Queue("test.queue".to_string());
        let msg = tibco_ems::TextMessage {
            body: "Hello World!".to_string(),
            ..Default::default()
        };
        session.send_message(&queue, msg).unwrap();

        let consumer = session.queue_consumer(&queue, None).unwrap();
        let msg = consumer.receive_message(Some(1)).unwrap().unwrap();
        match &msg {
            tibco_ems::Message::TextMessage(m) => {
                assert_eq!(m.body, "Hello World!");
            }
            _ => {
                panic!("Expected TextMessage");
            }
        }
        Ok(())
    }
}
