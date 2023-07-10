#[cfg(test)]
mod messages {
    use std::collections::HashMap;

    use tibco_ems::{Message, TypedValue};

    #[test]
    fn text_message_default() {
        let gen_msg = tibco_ems::TextMessage {
            ..Default::default()
        };
        let control_msg = tibco_ems::TextMessage {
            header: None,
            body: String::from(""),
            destination: None,
            reply_to: None,
            pointer: None
        };
        assert_eq!(gen_msg, control_msg)
    }

    #[test]
    fn bytes_message_default() {
        let gen_msg = tibco_ems::BytesMessage {
            ..Default::default()
        };
        let control_msg = tibco_ems::BytesMessage {
            header: None,
            body: String::from("").into_bytes(),
            destination: None,
            reply_to: None,
            pointer: None
        };
        assert_eq!(gen_msg, control_msg)
    }

    #[test]
    fn map_message_default() { 
        let gen_msg = tibco_ems::MapMessage {
            ..Default::default()
        };
        let body: HashMap<String, TypedValue> = HashMap::new();
        let control_msg = tibco_ems::MapMessage {
            header: None,
            body,
            destination: None,
            reply_to: None,
            pointer: None
        };
        assert_eq!(gen_msg, control_msg)
    }

    #[test]
    fn object_message_default() {
        let gen_msg = tibco_ems::ObjectMessage {
            ..Default::default()
        };
        let control_msg = tibco_ems::ObjectMessage {
            header: None,
            body: String::from("").into_bytes(),
            destination: None,
            reply_to: None,
            pointer: None
        };
        assert_eq!(gen_msg, control_msg)
    }

    #[test]
    fn text_message_clone() {
        let msg = tibco_ems::TextMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn map_message_clone() {
        let msg = tibco_ems::MapMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn bytes_message_clone() {
        let msg = tibco_ems::BytesMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn object_message_clone() {
        let msg = tibco_ems::ObjectMessage {
            pointer: Some(5),
            ..Default::default()
        };
        let msg2 = msg.clone();
        assert_eq!(msg2.pointer, None)
    }

    #[test]
    fn object_message_display() {
        let msg = tibco_ems::ObjectMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "ObjectMessage")
    }

    #[test]
    fn bytes_message_display() {
        let msg = tibco_ems::BytesMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "BytesMessage")
    }

    #[test]
    fn map_message_display() {
        let msg = tibco_ems::MapMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "MapMessage")
    }

    #[test]
    fn text_message_display() {
        let msg = tibco_ems::TextMessage {
            pointer: None,
            ..Default::default()
        };
        let msg2: Message = msg.into();
        let str = format!("{}", msg2);
        assert_eq!(str, "TextMessage")
    }
}
