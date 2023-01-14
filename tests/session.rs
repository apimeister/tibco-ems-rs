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
