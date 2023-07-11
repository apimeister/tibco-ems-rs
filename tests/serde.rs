#[cfg(feature = "serde")]
#[cfg(test)]
mod serde {
    use std::collections::HashMap;

    use tibco_ems::admin::{BridgeInfo, OverflowPolicy};
    use tibco_ems::admin::{QueueInfo, TopicInfo};
    use tibco_ems::{BytesMessage, Destination, TextMessage, TypedValue, ObjectMessage, MapMessage, Message};

    #[test]
    fn test_queue_info_serde() {
        let queue_info = QueueInfo {
            name: "my_queue".to_string(),
            pending_messages: Some(10),
            max_messages: Some(100),
            max_bytes: Some(1024),
            overflow_policy: Some(OverflowPolicy::Default),
            failsafe: Some(true),
            secure: Some(true),
            global: Some(false),
            sender_name: Some(true),
            sender_name_enforced: Some(false),
            prefetch: Some(50),
            expiry_override: Some(3600),
            redelivery_delay: Some(5000),
            consumer_count: Some(5),
            incoming_total_count: Some(1000),
            outgoing_total_count: Some(2000),
        };

        // Serialize the QueueInfo to JSON
        let json = serde_json::to_string(&queue_info).unwrap();

        // Deserialize the JSON back into a QueueInfo
        let deserialized: QueueInfo = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized QueueInfo is equal to the original
        assert_eq!(deserialized, queue_info);
    }

    #[test]
    fn test_topic_info_serde() {
        let topic_info = TopicInfo {
            name: "my_topic".to_string(),
            expiry_override: Some(3600),
            global: Some(false),
            max_bytes: Some(1024),
            max_messages: Some(100),
            overflow_policy: Some(OverflowPolicy::Default),
            prefetch: Some(50),
            durable_count: Some(5),
            subscriber_count: Some(10),
            pending_messages: Some(20),
            incoming_total_count: Some(1000),
            outgoing_total_count: Some(2000),
        };

        // Serialize the TopicInfo to JSON
        let json = serde_json::to_string(&topic_info).unwrap();

        // Deserialize the JSON back into a TopicInfo
        let deserialized: TopicInfo = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized TopicInfo is equal to the original
        assert_eq!(deserialized, topic_info);
    }

    #[test]
    fn test_destination_serde() {
        let queue_destination = Destination::Queue("my_queue".to_string());
        let topic_destination = Destination::Topic("my_topic".to_string());

        // Serialize the queue destination to JSON
        let queue_json = serde_json::to_string(&queue_destination).unwrap();

        // Serialize the topic destination to JSON
        let topic_json = serde_json::to_string(&topic_destination).unwrap();

        // Deserialize the queue JSON back into a Destination
        let deserialized_queue: Destination = serde_json::from_str(&queue_json).unwrap();

        // Deserialize the topic JSON back into a Destination
        let deserialized_topic: Destination = serde_json::from_str(&topic_json).unwrap();

        // Ensure that the deserialized destinations are equal to the original ones
        assert_eq!(deserialized_queue, queue_destination);
        assert_eq!(deserialized_topic, topic_destination);
    }

    #[test]
    fn test_text_message_serde() {
        let mut header = HashMap::new();
        header.insert(
            "header_key".to_string(),
            TypedValue::String("header_value".to_string()),
        );

        let text_message = TextMessage {
            body: "Hello, world!".to_string(),
            header: Some(header.clone()),
            destination: Some(Destination::Queue("my_queue".to_string())),
            reply_to: Some(Destination::Topic("my_topic".to_string())),
            pointer: Some(123),
        };

        // Serialize the TextMessage to JSON
        let json = serde_json::to_string(&text_message).unwrap();

        // Deserialize the JSON back into a TextMessage
        let deserialized: TextMessage = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized TextMessage is equal to the original
        assert_eq!(deserialized, text_message);
    }

    #[test]
    fn test_overflow_policy_serde() {
        let default_policy: OverflowPolicy = OverflowPolicy::Default;
        let discard_old_policy: OverflowPolicy = OverflowPolicy::DiscardOld;
        let reject_incoming_policy: OverflowPolicy = OverflowPolicy::RejectIncoming;

        // Serialize the OverflowPolicy enum variants to JSON
        let default_json = serde_json::to_string(&default_policy).unwrap();
        let discard_old_json = serde_json::to_string(&discard_old_policy).unwrap();
        let reject_incoming_json = serde_json::to_string(&reject_incoming_policy).unwrap();

        // Deserialize the JSON back into OverflowPolicy enum variants
        let deserialized_default: OverflowPolicy = serde_json::from_str(&default_json).unwrap();
        let deserialized_discard_old: OverflowPolicy =
            serde_json::from_str(&discard_old_json).unwrap();
        let deserialized_reject_incoming: OverflowPolicy =
            serde_json::from_str(&reject_incoming_json).unwrap();

        // Ensure that the deserialized OverflowPolicy variants are equal to the original ones
        assert_eq!(deserialized_default, default_policy);
        assert_eq!(deserialized_discard_old, discard_old_policy);
        assert_eq!(deserialized_reject_incoming, reject_incoming_policy);
    }

    #[test]
    fn test_bridge_info_serde() {
        let bridge_info = BridgeInfo {
            source: Destination::Queue("source_queue".to_string()),
            target: Destination::Topic("target_topic".to_string()),
            selector: Some("some_selector".to_string()),
        };

        // Serialize the BridgeInfo to JSON
        let json = serde_json::to_string(&bridge_info).unwrap();

        // Deserialize the JSON back into a BridgeInfo
        let deserialized: BridgeInfo = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized BridgeInfo is equal to the original
        assert_eq!(deserialized, bridge_info);
    }

    #[test]
    fn test_bytes_message_serde() {
        let mut header = HashMap::new();
        header.insert(
            "header_key".to_string(),
            TypedValue::String("header_value".to_string()),
        );

        let bytes_message = BytesMessage {
            body: vec![1, 2, 3, 4, 5],
            header: Some(header.clone()),
            destination: Some(Destination::Queue("my_queue".to_string())),
            reply_to: Some(Destination::Topic("my_topic".to_string())),
            pointer: Some(123),
        };

        // Serialize the BytesMessage to JSON
        let json = serde_json::to_string(&bytes_message).unwrap();

        // Deserialize the JSON back into a BytesMessage
        let deserialized: BytesMessage = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized BytesMessage is equal to the original
        assert_eq!(deserialized, bytes_message);
    }

    #[test]
    fn test_object_message_serde() {
        let mut header = HashMap::new();
        header.insert("header_key".to_string(), TypedValue::String("header_value".to_string()));

        let object_message = ObjectMessage {
            body: vec![1, 2, 3, 4, 5],
            header: Some(header.clone()),
            destination: Some(Destination::Queue("my_queue".to_string())),
            reply_to: Some(Destination::Topic("my_topic".to_string())),
            pointer: Some(123),
        };

        // Serialize the ObjectMessage to JSON
        let json = serde_json::to_string(&object_message).unwrap();

        // Deserialize the JSON back into an ObjectMessage
        let deserialized: ObjectMessage = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized ObjectMessage is equal to the original
        assert_eq!(deserialized, object_message);
    }

    #[test]
    fn test_map_message_serde() {
        let mut body = HashMap::new();
        body.insert("key1".to_string(), TypedValue::String("value1".to_string()));
        body.insert("key2".to_string(), TypedValue::Integer(42));

        let mut header = HashMap::new();
        header.insert("header_key".to_string(), TypedValue::String("header_value".to_string()));

        let map_message = MapMessage {
            body,
            header: Some(header.clone()),
            destination: Some(Destination::Queue("my_queue".to_string())),
            reply_to: Some(Destination::Topic("my_topic".to_string())),
            pointer: Some(123),
        };

        // Serialize the MapMessage to JSON
        let json = serde_json::to_string(&map_message).unwrap();

        // Deserialize the JSON back into a MapMessage
        let deserialized: MapMessage = serde_json::from_str(&json).unwrap();

        // Ensure that the deserialized MapMessage is equal to the original
        assert_eq!(deserialized, map_message);
    }

    #[test]
    fn test_message_serde() {
        let text_message = TextMessage {
            body: "Hello, world!".to_string(),
            header: None,
            destination: Some(Destination::Queue("my_queue".to_string())),
            reply_to: None,
            pointer: None,
        };

        let message: Message = text_message.clone().into();
        // Serialize the Message to JSON
        let json = serde_json::to_string(&message).unwrap();
        // Deserialize the JSON back into a Message
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        // Ensure that the deserialized Message is equal to the original
        assert_eq!(deserialized, message);

        if let Message::TextMessage(ref deserialized_text_message) = deserialized {
            assert_eq!(deserialized_text_message, &text_message);
        } else {
            panic!("Deserialized message is not a TextMessage variant");
        }

        let bytes_message = BytesMessage {
            body: vec![1, 2, 3, 4, 5],
            header: None,
            destination: Some(Destination::Topic("my_topic".to_string())),
            reply_to: None,
            pointer: None,
        };

        let message: Message = bytes_message.clone().into();
        // Serialize the Message to JSON
        let json = serde_json::to_string(&message).unwrap();
        // Deserialize the JSON back into a Message
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        // Ensure that the deserialized Message is equal to the original
        assert_eq!(deserialized, message);

        if let Message::BytesMessage(ref deserialized_bytes_message) = deserialized {
            assert_eq!(deserialized_bytes_message, &bytes_message);
        } else {
            panic!("Deserialized message is not a TextMessage variant");
        }

        let map_message = MapMessage {
            body: HashMap::new(),
            header: None,
            destination: Some(Destination::Queue("other_queue".to_string())),
            reply_to: None,
            pointer: None,
        };

        let message: Message = map_message.clone().into();
        // Serialize the Message to JSON
        let json = serde_json::to_string(&message).unwrap();
        // Deserialize the JSON back into a Message
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        // Ensure that the deserialized Message is equal to the original
        assert_eq!(deserialized, message);

        if let Message::MapMessage(ref deserialized_map_message) = deserialized {
            assert_eq!(deserialized_map_message, &map_message);
        } else {
            panic!("Deserialized message is not a TextMessage variant");
        }

        let object_message = ObjectMessage {
            body: vec![],
            header: None,
            destination: Some(Destination::Topic("other_topic".to_string())),
            reply_to: None,
            pointer: None,
        };

        let message: Message = object_message.clone().into();
        // Serialize the Message to JSON
        let json = serde_json::to_string(&message).unwrap();
        // Deserialize the JSON back into a Message
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        // Ensure that the deserialized Message is equal to the original
        assert_eq!(deserialized, message);

        if let Message::ObjectMessage(ref deserialized_object_message) = deserialized {
            assert_eq!(deserialized_object_message, &object_message);
        } else {
            panic!("Deserialized message is not a TextMessage variant");
        }
    }
}
