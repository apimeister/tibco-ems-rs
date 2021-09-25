#[cfg(test)]
mod tests {

  #[test]
  #[cfg(not(feature = "mock"))]
  fn test_connection_failure() -> Result<(), String> {
    let result = tibco_ems::connect("tcp://example.org:7222", "admin", "admin");
    match result {
      Ok(_val) => {
        return Err("no error was returned".to_string());
      }
      Err(_err) => {
        return Ok(());
      }
    }
  }

  #[test]
  #[cfg(feature = "mock")]
  fn test_mock_connection() -> Result<(), String> {
    let result = tibco_ems::connect("tcp://example.org:7222", "admin", "admin");
    match result {
      Ok(_val) => {
        return Ok(());
      }
      Err(_err) => {
        return Err("no error was returned".to_string());
      }
    }
  }
}
