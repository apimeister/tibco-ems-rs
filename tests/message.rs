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
    use tibco_ems::{BytesMessage, MapMessage, Message, ObjectMessage, TextMessage, TypedValue};

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

    #[test]
    fn test_typed_value_string() {
        let value = TypedValue::String("Hello".to_string());
        let formatted = format!("{}", value);
        assert_eq!(formatted, "Hello");
    }

    #[test]
    fn test_typed_value_boolean() {
        let value = TypedValue::Boolean(true);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "true");
    }

    #[test]
    fn test_typed_value_integer() {
        let value = TypedValue::Integer(42);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "42");
    }

    #[test]
    fn test_typed_value_long() {
        let value = TypedValue::Long(1234567890);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "1234567890");
    }

    #[test]
    fn test_typed_value_float() {
        let value = TypedValue::Float(3.14);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "3.14");
    }

    #[test]
    fn test_typed_value_double() {
        let value = TypedValue::Double(2.71828);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "2.71828");
    }

    #[test]
    fn test_typed_value_binary() {
        let value = TypedValue::Binary(vec![0x01, 0x02, 0x03]);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "[1, 2, 3]");
    }

    #[test]
    fn test_typed_value_map() {
        let map_message = MapMessage {
            ..Default::default()
        };
        let value = TypedValue::Map(map_message);
        let formatted = format!("{}", value);
        assert_eq!(formatted, "MapMessage { body: {}, header: None, destination: None, reply_to: None, pointer: None }");
    }

    // Helper function to compare formatted output with expected value
    fn assert_display<T: std::fmt::Display>(value: T, expected: &str) {
        let formatted = format!("{}", value);
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_typed_value_display_helper() {
        assert_display(TypedValue::String("Hello".to_string()), "Hello");
        assert_display(TypedValue::Boolean(true), "true");
        assert_display(TypedValue::Integer(42), "42");
        assert_display(TypedValue::Long(1234567890), "1234567890");
        assert_display(TypedValue::Float(3.14), "3.14");
        assert_display(TypedValue::Double(2.71828), "2.71828");
        assert_display(TypedValue::Binary(vec![0x01, 0x02, 0x03]), "[1, 2, 3]");
        let map_message = MapMessage {
            ..Default::default()
        };
        assert_display(TypedValue::Map(map_message), "MapMessage { body: {}, header: None, destination: None, reply_to: None, pointer: None }");
    }
}
