#[cfg(all(feature = "ems-sys", feature = "integration-tests"))]
#[cfg(test)]
mod session_struct {

    use tibco_ems::{Destination, TextMessage};

    const USER: &str = "admin";
    const PASSWORD: &str = "";
    const URL: &str = "tcp://localhost:7222";

    #[test]
    fn test_queue_consumer_with_queue_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-success".into());
        let queue_consumer = session.queue_consumer(&dest, None);
        assert!(queue_consumer.is_ok());
    }
    #[test]
    fn test_queue_consumer_with_queue_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-failure".into());
        let queue_consumer = session.queue_consumer(&dest, None);
        assert!(queue_consumer.is_err());
    }

    #[test]
    fn test_queue_consumer_with_topic_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-success".into());
        let queue_consumer = session.queue_consumer(&dest, None);
        assert!(queue_consumer.is_ok());
    }
    #[test]
    fn test_queue_consumer_with_topic_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-failure".into());
        let queue_consumer = session.queue_consumer(&dest, None);
        assert!(queue_consumer.is_err());
    }

    #[test]
    fn test_topic_consumer_with_topic_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-success".into());
        let topic_consumer = session.topic_consumer(&dest, "test-1", None);
        assert!(topic_consumer.is_ok());
    }
    #[test]
    fn test_topic_consumer_with_topic_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-failure".into());
        let topic_consumer = session.topic_consumer(&dest, "test-2", None);
        assert!(topic_consumer.is_err());
    }
    #[test]
    fn test_topic_consumer_with_queue_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-failure".into());
        let topic_consumer = session.topic_consumer(&dest, "test-3", None);
        assert!(topic_consumer.is_err());
    }

    #[test]
    fn test_topic_durable_consumer_with_topic_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-success".into());
        let topic_durable_consumer = session.topic_durable_consumer(&dest, "test-durable-1", None);
        assert!(topic_durable_consumer.is_ok());
    }
    #[test]
    fn test_topic_durable_consumer_with_topic_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-failure".into());
        let topic_durable_consumer = session.topic_durable_consumer(&dest, "test-durable-2", None);
        assert!(topic_durable_consumer.is_err());
    }
    #[test]
    fn test_topic_durable_consumer_with_queue_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-failure".into());
        let topic_durable_consumer = session.topic_durable_consumer(&dest, "test-durable-3", None);
        assert!(topic_durable_consumer.is_err());
    }

    #[test]
    fn test_send_message_to_queue_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-success".into());
        let msg = TextMessage {
            body: "hello".into(),
            ..Default::default()
        };
        let msg_sent = session.send_message(&dest, msg);
        assert!(msg_sent.is_ok());
    }

    #[test]
    fn test_send_message_to_queue_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-failure".into());
        let msg = TextMessage {
            body: "hello".into(),
            ..Default::default()
        };
        let msg_sent = session.send_message(&dest, msg);
        assert!(msg_sent.is_err());
    }

    #[test]
    fn test_send_message_to_topic_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-success".into());
        let msg = TextMessage {
            body: "hello".into(),
            ..Default::default()
        };
        let msg_sent = session.send_message(&dest, msg);
        assert!(msg_sent.is_ok());
    }

    #[test]
    fn test_send_message_to_topic_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-failure".into());
        let msg = TextMessage {
            body: "hello".into(),
            ..Default::default()
        };
        let msg_sent = session.send_message(&dest, msg);
        assert!(msg_sent.is_err());
    }

    #[test]
    fn test_request_reply_queue_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Queue("test-success".into());
        let msg = TextMessage {
            body: "hello".into(),
            ..Default::default()
        };
        let msg_sent = session.request_reply(&dest, msg, 5);
        assert!(msg_sent.is_ok());
        let unwrapped_msg = msg_sent.unwrap();
        assert!(unwrapped_msg.is_none());
    }

    // Error Case not implemented in function

    // #[test]
    // fn test_request_reply_queue_failure() {
    //     let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
    //     let session = con.session().unwrap();
    //     let dest = Destination::Queue("test-failure".into());
    //     let msg = TextMessage {
    //         body: "hello".into(),
    //         ..Default::default()
    //     };
    //     let msg_sent = session.request_reply(&dest, msg, 5);
    //     println!("{:?}", msg_sent);
    //     assert!(msg_sent.is_err());
    // }

    #[test]
    fn test_request_reply_topic_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let dest = Destination::Topic("test-success".into());
        let msg = TextMessage {
            body: "hello".into(),
            ..Default::default()
        };
        let msg_sent = session.request_reply(&dest, msg, 5);
        assert!(msg_sent.is_ok());
        let unwrapped_msg = msg_sent.unwrap();
        assert!(unwrapped_msg.is_none());
    }

    // Error Case not implemented in function

    // #[test]
    // fn test_request_reply_topic_failure() {
    //     let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
    //     let session = con.session().unwrap();
    //     let dest = Destination::Topic("test-failure".into());
    //     let msg = TextMessage {
    //         body: "hello".into(),
    //         ..Default::default()
    //     };
    //     let msg_sent = session.request_reply(&dest, msg, 5);
    //     println!("{:?}", msg_sent);
    //     assert!(msg_sent.is_err());
    // }
}

#[cfg(test)]
#[cfg(feature = "mock")]
mod tests {

    #[test]
    fn test_mock_session() -> Result<(), String> {
        let conn = tibco_ems::connect("tcp://example.org:7222", "admin", "admin").unwrap();
        let session = conn.session();
        match session {
            Ok(_val) => {
                return Ok(());
            }
            Err(_err) => {
                return Err("no error was returned".to_string());
            }
        }
    }
}
