#[cfg(test)]
mod text_message {
    use std::collections::HashMap;
    use tibco_ems::{Destination, Message, TextMessage, TypedValue};

    #[test]
    fn test_text_message_default() {
        let text_message = TextMessage::default();

        // Ensure that the default values are set correctly
        assert_eq!(text_message.body, String::new());
        assert_eq!(text_message.header, None);
        assert_eq!(text_message.destination, None);
        assert_eq!(text_message.reply_to, None);
        assert_eq!(text_message.pointer, None);
    }

    #[test]
    fn test_text_message_creation() {
        // Create sample values for the fields
        let body = "Hello, world!".to_string();

        let mut header = HashMap::new();
        header.insert(
            "header_key".to_string(),
            TypedValue::String("header_value".to_string()),
        );

        let destination = Some(Destination::Queue("test1".to_string()));
        let reply_to = Some(Destination::Queue("test1".to_string()));
        let pointer = Some(123);

        // Create a TextMessage instance
        let text_message = TextMessage {
            body: body.clone(),
            header: Some(header.clone()),
            destination,
            reply_to,
            pointer,
        };

        // Ensure that the fields are set correctly
        assert_eq!(text_message.body, body);
        assert_eq!(text_message.header, Some(header));
        assert_eq!(
            text_message.destination,
            Some(Destination::Queue("test1".to_string()))
        );
        assert_eq!(
            text_message.reply_to,
            Some(Destination::Queue("test1".to_string()))
        );
        assert_eq!(text_message.pointer, Some(123));
    }

    #[test]
    fn test_text_message_clone() {
        let msg = TextMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn test_text_message_display() {
        let msg = TextMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "TextMessage")
    }
}

#[cfg(test)]
mod bytes_message {
    use std::collections::HashMap;

    use tibco_ems::{BytesMessage, Destination, Message, TypedValue};

    #[test]
    fn test_bytes_message_default() {
        let bytes_message = BytesMessage::default();

        // Ensure that the default values are set correctly
        assert_eq!(bytes_message.body, Vec::new());
        assert_eq!(bytes_message.header, None);
        assert_eq!(bytes_message.destination, None);
        assert_eq!(bytes_message.reply_to, None);
        assert_eq!(bytes_message.pointer, None);
    }

    #[test]
    fn test_bytes_message_creation() {
        // Create sample values for the fields
        let body = vec![1, 2, 3, 4];

        let mut header = HashMap::new();
        header.insert(
            "header_key".to_string(),
            TypedValue::String("header_value".to_string()),
        );

        let destination = Some(Destination::Topic("test1".to_string()));
        let reply_to = Some(Destination::Topic("test1".to_string()));
        let pointer = Some(123);

        // Create a BytesMessage instance
        let bytes_message = BytesMessage {
            body: body.clone(),
            header: Some(header.clone()),
            destination,
            reply_to,
            pointer,
        };

        // Ensure that the fields are set correctly
        assert_eq!(bytes_message.body, body);
        assert_eq!(bytes_message.header, Some(header));
        assert_eq!(
            bytes_message.destination,
            Some(Destination::Topic("test1".to_string()))
        );
        assert_eq!(
            bytes_message.reply_to,
            Some(Destination::Topic("test1".to_string()))
        );
        assert_eq!(bytes_message.pointer, Some(123));
    }

    #[test]
    fn test_bytes_message_clone() {
        let msg = BytesMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn test_bytes_message_display() {
        let msg = BytesMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "BytesMessage")
    }
}

#[cfg(test)]
mod object_message {
    use std::collections::HashMap;

    use tibco_ems::{Destination, Message, ObjectMessage, TypedValue};

    #[test]
    fn test_object_message_default() {
        let object_message = ObjectMessage::default();

        // Ensure that the default values are set correctly
        assert_eq!(object_message.body, Vec::new());
        assert_eq!(object_message.header, None);
        assert_eq!(object_message.destination, None);
        assert_eq!(object_message.reply_to, None);
        assert_eq!(object_message.pointer, None);
    }

    #[test]
    fn test_object_message_creation() {
        // Create sample values for the fields
        let body = vec![1, 2, 3, 4];

        let mut header = HashMap::new();
        header.insert(
            "header_key".to_string(),
            TypedValue::String("header_value".to_string()),
        );

        let destination = Some(Destination::Topic("test1".to_string()));
        let reply_to = Some(Destination::Topic("test1".to_string()));
        let pointer = Some(123);

        // Create an ObjectMessage instance
        let object_message = ObjectMessage {
            body: body.clone(),
            header: Some(header.clone()),
            destination,
            reply_to,
            pointer,
        };

        // Ensure that the fields are set correctly
        assert_eq!(object_message.body, body);
        assert_eq!(object_message.header, Some(header));
        assert_eq!(
            object_message.destination,
            Some(Destination::Topic("test1".to_string()))
        );
        assert_eq!(
            object_message.reply_to,
            Some(Destination::Topic("test1".to_string()))
        );
        assert_eq!(object_message.pointer, Some(123));
    }
    #[test]
    fn test_object_message_clone() {
        let msg = ObjectMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn test_object_message_display() {
        let msg = ObjectMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "ObjectMessage")
    }
}

#[cfg(test)]
mod map_message {
    use std::collections::HashMap;

    use tibco_ems::{Destination, MapMessage, Message, TypedValue};

    #[test]
    fn test_map_message_default() {
        let map_message = MapMessage::default();

        // Ensure that the default values are set correctly
        assert_eq!(map_message.body, HashMap::new());
        assert_eq!(map_message.header, None);
        assert_eq!(map_message.destination, None);
        assert_eq!(map_message.reply_to, None);
        assert_eq!(map_message.pointer, None);
    }

    #[test]
    fn test_map_message_creation() {
        // Create sample values for the fields
        let mut body = HashMap::new();
        body.insert("key".to_string(), TypedValue::Integer(42));

        let mut header = HashMap::new();
        header.insert(
            "header_key".to_string(),
            TypedValue::String("header_value".to_string()),
        );

        let destination = Some(Destination::Queue("test1".to_string()));
        let reply_to = Some(Destination::Queue("test1".to_string()));
        let pointer = Some(123);

        // Create a MapMessage instance
        let map_message = MapMessage {
            body: body.clone(),
            header: Some(header.clone()),
            destination,
            reply_to,
            pointer,
        };

        // Ensure that the fields are set correctly
        assert_eq!(map_message.body, body);
        assert_eq!(map_message.header, Some(header));
        assert_eq!(
            map_message.destination,
            Some(Destination::Queue("test1".to_string()))
        );
        assert_eq!(
            map_message.reply_to,
            Some(Destination::Queue("test1".to_string()))
        );
        assert_eq!(map_message.pointer, Some(123));
    }

    #[test]
    fn test_map_message_clone() {
        let msg = MapMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn test_map_message_display() {
        let msg = tibco_ems::MapMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "MapMessage")
    }
}

#[cfg(test)]
mod message {
    use tibco_ems::{BytesMessage, MapMessage, Message, ObjectMessage, TextMessage};

    #[test]
    fn test_text_message_variant() {
        let text_message = TextMessage {
            ..Default::default()
        };
        let message = Message::TextMessage(text_message.clone());

        assert_eq!(message, Message::TextMessage(text_message));
    }

    #[test]
    fn test_bytes_message_variant() {
        let bytes_message = BytesMessage {
            ..Default::default()
        };
        let message = Message::BytesMessage(bytes_message.clone());

        assert_eq!(message, Message::BytesMessage(bytes_message));
    }

    #[test]
    fn test_object_message_variant() {
        let object_message = ObjectMessage {
            ..Default::default()
        };
        let message = Message::ObjectMessage(object_message.clone());

        assert_eq!(message, Message::ObjectMessage(object_message));
    }

    #[test]
    fn test_map_message_variant() {
        let map_message = MapMessage {
            ..Default::default()
        };
        let message = Message::MapMessage(map_message.clone());

        assert_eq!(message, Message::MapMessage(map_message));
    }
}
