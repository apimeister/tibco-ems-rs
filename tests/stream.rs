#[cfg(all(feature = "integration-tests", feature = "streaming"))]
#[cfg(test)]
mod stream {
    use tibco_ems::{Destination, TextMessage};

    const USER: &str = "admin";
    const PASSWORD: &str = "";
    const URL: &str = "tcp://localhost:7222";

    #[test]
    fn test_open_stream_failure() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        // for this to work "test-failure" queue may not be present on the server
        let dest = Destination::Queue("test-failure".into());
        let stream = con.open_stream::<TextMessage>(&dest, None);
        assert!(stream.is_err());
    }

    #[test]
    fn test_open_stream_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        // for this to work "test-success" queue needs to be present on the server
        let dest = Destination::Queue("test-success".into());
        let stream = con.open_stream::<TextMessage>(&dest, None);
        assert!(stream.is_ok());
    }
}
