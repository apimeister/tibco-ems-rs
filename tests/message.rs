#[cfg(test)]
mod messages {

  #[test]
  fn text_message_default() -> Result<(), String> {
    let _msg = tibco_ems::TextMessage{
      ..Default::default()
    };
    Ok(())
  }

  #[test]
  fn text_message_body() -> Result<(), String> {
    let _msg = tibco_ems::TextMessage{
      body: "Hello World".to_string(),
      ..Default::default()
    };
    Ok(())
  }

  #[test]
  fn bytes_message_default() -> Result<(), String> {
    let _msg = tibco_ems::BytesMessage{
      ..Default::default()
    };
    Ok(())
  }

  #[test]
  fn bytes_message_body() -> Result<(), String> {
    let _msg = tibco_ems::BytesMessage{
      body: vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09],
      ..Default::default()
    };
    Ok(())
  }

}
