#[cfg(test)]
mod messages {
    use tibco_ems::Message;

    #[test]
    fn text_message_default() -> Result<(), String> {
        let _msg = tibco_ems::TextMessage {
            ..Default::default()
        };
        Ok(())
    }

    #[test]
    fn text_message_body() -> Result<(), String> {
        let _msg = tibco_ems::TextMessage {
            body: "Hello World".to_string(),
            ..Default::default()
        };
        Ok(())
    }

    #[test]
    fn bytes_message_default() -> Result<(), String> {
        let _msg = tibco_ems::BytesMessage {
            ..Default::default()
        };
        Ok(())
    }

    #[test]
    fn bytes_message_body() -> Result<(), String> {
        let _msg = tibco_ems::BytesMessage {
            body: vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09],
            ..Default::default()
        };
        Ok(())
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
