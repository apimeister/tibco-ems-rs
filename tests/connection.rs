#[cfg(feature = "ems-sys")]
#[cfg(test)]
mod connection {

    #[test]
    fn test_connection_failure() {
        // for this to work authentication needs to be enabled on the server
        let result = tibco_ems::connect("wrong url", "admin", "");
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "integration-tests")]
    fn test_connection_success() {
        let result = tibco_ems::connect("tcp://localhost:7222", "admin", "");
        assert!(result.is_ok());
    }
}

#[cfg(all(feature = "ems-sys", feature = "integration-tests"))]
#[cfg(test)]
mod connection_struct {

    const USER: &str = "admin";
    const PASSWORD: &str = "";
    const URL: &str = "tcp://localhost:7222";

    // FIXME: no idea how to get into the failure state
    // #[test]
    // fn test_session_failure() {
    //     let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
    //     let session = con.session();
    //     assert!(session.is_err());
    // }

    #[test]
    fn test_session_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session();
        assert!(session.is_ok());
    }

    // FIXME: no idea how to get into the failure state
    // #[test]
    // fn test_session_failure() {
    //     let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
    //     let session = con.transacted_session();
    //     assert!(session.is_err());
    // }

    #[test]
    fn test_transacted_session_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let session = con.transacted_session();
        assert!(session.is_ok());
    }

    #[test]
    fn test_get_active_url_success() {
        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        let url = con.get_active_url();
        assert_eq!(url.unwrap(), "tcp://localhost:7222");
    }

    #[cfg(feature = "streaming")]
    #[test]
    fn test_open_stream_failure() {
        use tibco_ems::{Destination, TextMessage};

        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        // for this to work "test-failure" queue may not be present on the server
        let dest = Destination::Queue("test-failure".into());
        let stream = con.open_stream::<TextMessage>(&dest, None);
        assert!(stream.is_err());
    }

    #[cfg(feature = "streaming")]
    #[test]
    fn test_open_stream_success() {
        use tibco_ems::{Destination, TextMessage};

        let con = tibco_ems::connect(URL, USER, PASSWORD).unwrap();
        // for this to work "test-success" queue needs to be present on the server
        let dest = Destination::Queue("test-success".into());
        let stream = con.open_stream::<TextMessage>(&dest, None);
        assert!(stream.is_ok());
    }
}

#[cfg(not(feature = "ems-sys"))]
#[cfg(test)]
mod tests {
    #[test]

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
