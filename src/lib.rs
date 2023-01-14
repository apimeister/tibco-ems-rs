#![warn(missing_docs)]
//! Tibco EMS binding.

#[cfg(feature = "ems-sys")]
use enum_extract::extract;
#[cfg(feature = "ems-sys")]
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "ems-sys")]
use std::ffi::c_void;
#[cfg(feature = "ems-sys")]
use std::ffi::CStr;
#[cfg(feature = "ems-sys")]
use std::ffi::CString;
use std::fmt;
use std::io::Error;
#[cfg(feature = "ems-sys")]
use std::io::ErrorKind;
#[cfg(feature = "ems-sys")]
use std::ops::Deref;
use std::sync::Arc;
#[cfg(feature = "ems-sys")]
use tibco_ems_sys::{tibemsDestinationType, tibemsMsgType, tibems_bool, tibems_status};

pub mod admin;

#[cfg(feature = "streaming")]
pub mod stream;

/// holds the native Connection pointer
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Connection {
    pointer: Arc<usize>,
}

/// holds the native Session pointer
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Session {
    pointer: usize,
    producer_pointer: usize,
}

/// holds the native Consumer pointer
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Consumer {
    pointer: usize,
}

/// Destination, can either be Queue or Topic
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Destination {
    /// Destination type Queue
    Queue(String),
    /// Destination type Topic
    Topic(String),
}

/// represents a Text Message
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TextMessage {
    /// message body
    pub body: String,
    /// message header
    pub header: Option<HashMap<String, TypedValue>>,
    /// message destination
    pub destination: Option<Destination>,
    /// reply to header
    pub reply_to: Option<Destination>,
    /// point to the ems native object
    pub pointer: Option<usize>,
}

impl Clone for TextMessage {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            header: self.header.clone(),
            destination: self.destination.clone(),
            reply_to: self.reply_to.clone(),
            pointer: None,
        }
    }
}

/// represents a Binary Message
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BytesMessage {
    /// message body
    pub body: Vec<u8>,
    /// message header
    pub header: Option<HashMap<String, TypedValue>>,
    /// message destination
    pub destination: Option<Destination>,
    /// reply to header
    pub reply_to: Option<Destination>,
    /// point to the ems native object
    pub pointer: Option<usize>,
}

impl Clone for BytesMessage {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            header: self.header.clone(),
            destination: self.destination.clone(),
            reply_to: self.reply_to.clone(),
            pointer: None,
        }
    }
}

/// represents a Object Message
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ObjectMessage {
    /// message body
    pub body: Vec<u8>,
    /// message header
    pub header: Option<HashMap<String, TypedValue>>,
    /// message destination
    pub destination: Option<Destination>,
    /// reply to header
    pub reply_to: Option<Destination>,
    /// point to the ems native object
    pub pointer: Option<usize>,
}

impl Clone for ObjectMessage {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            header: self.header.clone(),
            destination: self.destination.clone(),
            reply_to: self.reply_to.clone(),
            pointer: None,
        }
    }
}

/// represents a Map Message
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct MapMessage {
    /// message body map properties
    pub body: HashMap<String, TypedValue>,
    /// message header
    pub header: Option<HashMap<String, TypedValue>>,
    /// message destination
    pub destination: Option<Destination>,
    /// reply to header
    pub reply_to: Option<Destination>,
    /// point to the ems native object
    pub pointer: Option<usize>,
}

impl Clone for MapMessage {
    fn clone(&self) -> Self {
        Self {
            body: self.body.clone(),
            header: self.header.clone(),
            destination: self.destination.clone(),
            reply_to: self.reply_to.clone(),
            pointer: None,
        }
    }
}

/// Message enum wich represents the different message types
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// represents a Text Message
    TextMessage(TextMessage),
    /// represents a Binary Message
    BytesMessage(BytesMessage),
    /// represents a Map Message
    MapMessage(MapMessage),
    /// represents a Object Message
    ObjectMessage(ObjectMessage),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::TextMessage(_) => write!(f, "TextMessage"),
            Message::BytesMessage(_) => write!(f, "BytesMessage"),
            Message::MapMessage(_) => write!(f, "MapMessage"),
            Message::ObjectMessage(_) => write!(f, "ObjectMessage"),
        }
    }
}

#[cfg(feature = "ems-sys")]
/// open a connection to the Tibco EMS server
pub fn connect(url: &str, user: &str, password: &str) -> Result<Connection, Error> {
    let mut connection_pointer: usize = 0;
    unsafe {
        let factory = tibco_ems_sys::tibemsConnectionFactory_Create();
        let c_url = CString::new(url).unwrap();
        let status = tibco_ems_sys::tibemsConnectionFactory_SetServerURL(factory, c_url.as_ptr());
        match status {
            tibems_status::TIBEMS_OK => {
                trace!("tibemsConnectionFactory_SetServerURL: {:?}", status)
            }
            _ => {
                error!("tibemsConnectionFactory_SetServerURL: {:?}", status);
                return Err(Error::new(ErrorKind::InvalidData, "cannot set server url"));
            }
        }
        let c_user = CString::new(user).unwrap();
        let c_password = CString::new(password).unwrap();
        let status = tibco_ems_sys::tibemsConnectionFactory_CreateConnection(
            factory,
            &mut connection_pointer,
            c_user.as_ptr(),
            c_password.as_ptr(),
        );
        match status {
            tibems_status::TIBEMS_OK => {
                trace!("tibemsConnectionFactory_CreateConnection: {:?}", status)
            }
            _ => {
                error!("tibemsConnectionFactory_CreateConnection: {:?}", status);
                return Err(Error::new(
                    ErrorKind::NotConnected,
                    "cannot create connection",
                ));
            }
        }
        let status = tibco_ems_sys::tibemsConnection_Start(connection_pointer);
        match status {
            tibems_status::TIBEMS_OK => trace!("tibemsConnection_Start: {:?}", status),
            _ => {
                error!("tibemsConnection_Start: {:?}", status);
                return Err(Error::new(
                    ErrorKind::NotConnected,
                    "cannot start connection",
                ));
            }
        }
    }
    let conn = Connection {
        pointer: Arc::from(connection_pointer),
    };
    Ok(conn)
}
#[cfg(not(feature = "ems-sys"))]
/// open a connection to the Tibco EMS server
pub fn connect(_url: &str, _user: &str, _password: &str) -> Result<Connection, Error> {
    let conn = Connection {
        pointer: Arc::from(0),
    };
    unsafe {
        SERVER.connection = Some(conn.clone());
    }
    Ok(conn)
}

#[cfg(not(feature = "ems-sys"))]
/// contains a MockServer to emulate a Tibco EMS server
pub struct MockServer {
    pub connection: Option<Connection>,
    pub session: Option<Session>,
    pub messages: Vec<(Destination, Message)>,
    pub consumer: Option<Destination>,
}
#[cfg(not(feature = "ems-sys"))]
pub static mut SERVER: MockServer = MockServer {
    connection: None,
    session: None,
    messages: vec![],
    consumer: None,
};

//
// connection
//

impl Connection {
    #[cfg(feature = "ems-sys")]
    /// open a session
    pub fn session(&self) -> Result<Session, Error> {
        unsafe {
            let mut session_pointer: usize = 0;
            let connection_pointer = *self.pointer.deref();
            let status = tibco_ems_sys::tibemsConnection_CreateSession(
                connection_pointer,
                &mut session_pointer,
                tibco_ems_sys::tibems_bool::TIBEMS_FALSE,
                tibco_ems_sys::tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE,
            );
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsConnection_CreateSession: {:?}", status),
                _ => {
                    error!("tibemsConnection_CreateSession: {:?}", status);
                    return Err(Error::new(ErrorKind::Other, "creating session failed"));
                }
            }
            let mut producer: usize = 0;
            let dest: usize = 0;
            let status =
                tibco_ems_sys::tibemsSession_CreateProducer(session_pointer, &mut producer, dest);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}", status),
                _ => {
                    error!("tibemsSession_CreateProducer: {:?}", status);
                    return Err(Error::new(ErrorKind::Other, "creating producer failed"));
                }
            }
            let session = Session {
                pointer: session_pointer,
                producer_pointer: producer,
            };
            Ok(session)
        }
    }
    #[cfg(not(feature = "ems-sys"))]
    /// open a session
    pub fn session(&self) -> Result<Session, Error> {
        Ok(Session {
            pointer: 0,
            producer_pointer: 0,
        })
    }
    #[cfg(feature = "ems-sys")]
    /// open a session with transaction support
    pub fn transacted_session(&self) -> Result<Session, Error> {
        let session: Session;
        unsafe {
            let mut session_pointer: usize = 0;
            let connection_pointer = *self.pointer.deref();
            let status = tibco_ems_sys::tibemsConnection_CreateSession(
                connection_pointer,
                &mut session_pointer,
                tibco_ems_sys::tibems_bool::TIBEMS_FALSE,
                tibco_ems_sys::tibemsAcknowledgeMode::TIBEMS_EXPLICIT_CLIENT_ACKNOWLEDGE,
            );
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsConnection_CreateSession: {:?}", status),
                _ => {
                    error!("tibemsConnection_CreateSession: {:?}", status);
                    return Err(Error::new(ErrorKind::Other, "creating session failed"));
                }
            }
            let mut producer: usize = 0;
            let dest: usize = 0;
            let status =
                tibco_ems_sys::tibemsSession_CreateProducer(session_pointer, &mut producer, dest);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}", status),
                _ => {
                    error!("tibemsSession_CreateProducer: {:?}", status);
                    return Err(Error::new(ErrorKind::Other, "creating producer failed"));
                }
            }
            session = Session {
                pointer: session_pointer,
                producer_pointer: producer,
            };
        }
        Ok(session)
    }

    #[cfg(not(feature = "ems-sys"))]
    /// open a session
    pub fn transacted_session(&self) -> Result<Session, Error> {
        Ok(Session {
            pointer: 0,
            producer_pointer: 0,
        })
    }

    #[cfg(feature = "ems-sys")]
    /// get active url from a ft connection
    /// this is only required for admin connections,
    /// normal connections automatically choose the active server
    pub fn get_active_url(&self) -> Result<String, Error> {
        let connection_pointer = *self.pointer.deref();
        unsafe {
            let buf_vec: Vec<i8> = vec![0; 0];
            let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
            let status = tibco_ems_sys::tibemsConnection_GetActiveURL(connection_pointer, &buf_ref);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsConnection_GetActiveURL: {:?}", status),
                _ => {
                    error!("tibemsConnection_GetActiveURL: {:?}", status);
                    return Err(Error::new(
                        ErrorKind::Other,
                        "failed to retrieve active url",
                    ));
                }
            }
            let url = CStr::from_ptr(buf_ref).to_str().unwrap();
            Ok(url.to_string())
        }
    }

    #[cfg(not(feature = "ems-sys"))]
    /// get active url from a ft connection
    /// this is only required for admin connections,
    /// normal connections automatically choose the active server
    pub fn get_active_url(&self) -> Result<String, Error> {
        Ok("".to_string())
    }

    // open a consumer as stream of messages
    #[cfg(feature = "streaming")]
    pub fn open_stream<'stream, T: Into<Message>>(
        &'stream self,
        destination: &Destination,
        selector: Option<&str>,
    ) -> Result<stream::MessageStream<T>, Error> {
        let session = self.session().unwrap();
        let consumer = session.queue_consumer(destination, selector).unwrap();
        let stream = stream::MessageStream::<T> {
            connection: std::rc::Rc::from(self.clone()),
            session: std::rc::Rc::from(session),
            consumer: std::rc::Rc::from(consumer),
            message: None,
        };
        Ok(stream)
    }
}

//
// consumer
//

impl Consumer {
    #[cfg(feature = "ems-sys")]
    /// receive messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// a wait time of None blocks until a message is available
    pub fn receive_message(&self, wait_time_ms: Option<i64>) -> Result<Option<Message>, Error> {
        unsafe {
            let mut msg_pointer: usize = 0;
            match wait_time_ms {
                Some(time_ms) => {
                    let status = tibco_ems_sys::tibemsMsgConsumer_ReceiveTimeout(
                        self.pointer,
                        &mut msg_pointer,
                        time_ms,
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsMsgConsumer_ReceiveTimeout: {:?}", status)
                        }
                        tibems_status::TIBEMS_TIMEOUT => {
                            return Ok(None);
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsMsgConsumer_ReceiveTimeout: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("receive message failed: {}", status_str),
                            ));
                        }
                    }
                }
                None => {
                    let status =
                        tibco_ems_sys::tibemsMsgConsumer_Receive(self.pointer, &mut msg_pointer);
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsMsgConsumer_Receive: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsMsgConsumer_Receive: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("receive message failed: {}", status_str),
                            ));
                        }
                    }
                }
            }
            let msg = build_message_from_pointer(msg_pointer);
            Ok(Some(msg))
        }
    }

    #[cfg(feature = "ems-sys")]
    /// receive text messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// a wait time of None blocks until a message is available
    pub fn receive_text_message(
        &self,
        wait_time_ms: Option<i64>,
    ) -> Result<Option<TextMessage>, Error> {
        let msg_option = self.receive_message(wait_time_ms)?;
        match msg_option {
            Some(msg) => match &msg {
                Message::TextMessage(text_msg) => Ok(Some(text_msg.to_owned())),
                _ => Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "received message with unexpected type (expected: TextMessage, found: {}",
                        msg
                    ),
                )),
            },
            None => Ok(None),
        }
    }

    #[cfg(feature = "ems-sys")]
    /// receive bytes messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// a wait time of None blocks until a message is available
    pub fn receive_bytes_message(
        &self,
        wait_time_ms: Option<i64>,
    ) -> Result<Option<BytesMessage>, Error> {
        let msg_option = self.receive_message(wait_time_ms)?;
        match msg_option {
            Some(msg) => match &msg {
                Message::BytesMessage(bytes_msg) => Ok(Some(bytes_msg.to_owned())),
                _ => Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "received message with unexpected type (expected: BytesMessage, found: {}",
                        msg
                    ),
                )),
            },
            None => Ok(None),
        }
    }

    #[cfg(feature = "ems-sys")]
    /// receive map messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// a wait time of None blocks until a message is available
    pub fn receive_map_message(
        &self,
        wait_time_ms: Option<i64>,
    ) -> Result<Option<MapMessage>, Error> {
        let msg_option = self.receive_message(wait_time_ms)?;
        match msg_option {
            Some(msg) => match &msg {
                Message::MapMessage(map_msg) => Ok(Some(map_msg.to_owned())),
                _ => Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "received message with unexpected type (expected: MapMessage, found: {}",
                        msg
                    ),
                )),
            },
            None => Ok(None),
        }
    }

    #[cfg(feature = "ems-sys")]
    /// receive object messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// a wait time of None blocks until a message is available
    pub fn receive_object_message(
        &self,
        wait_time_ms: Option<i64>,
    ) -> Result<Option<ObjectMessage>, Error> {
        let msg_option = self.receive_message(wait_time_ms)?;
        match msg_option {
            Some(msg) => match &msg {
                Message::ObjectMessage(object_msg) => Ok(Some(object_msg.to_owned())),
                _ => Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "received message with unexpected type (expected: ObjectMessage, found: {}",
                        msg
                    ),
                )),
            },
            None => Ok(None),
        }
    }

    #[cfg(not(feature = "ems-sys"))]
    /// receive messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// the wait time is ignored
    pub fn receive_message(&self, _wait_time_ms: Option<i64>) -> Result<Option<Message>, Error> {
        unsafe {
            let consumer_destination = SERVER.consumer.clone().unwrap();
            let messages = SERVER.messages.clone();
            for (dest, msg) in messages {
                if dest == consumer_destination {
                    return Ok(Some(msg));
                }
            }
        }
        Ok(None)
    }
    #[cfg(not(feature = "ems-sys"))]
    /// receive text messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// the wait time is ignored
    pub fn receive_text_message(
        &self,
        _wait_time_ms: Option<i64>,
    ) -> Result<Option<TextMessage>, Error> {
        unsafe {
            let consumer_destination = SERVER.consumer.clone().unwrap();
            let messages = SERVER.messages.clone();
            for (dest, msg) in messages {
                if dest == consumer_destination {
                    return match msg {
                        Message::TextMessage(ref msg) => Ok(Some(msg.to_owned())),
                        _ => Ok(None),
                    };
                };
            }
        }
        Ok(None)
    }

    #[cfg(not(feature = "ems-sys"))]
    /// receive bytes messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// the wait time is ignored
    pub fn receive_bytes_message(
        &self,
        _wait_time_ms: Option<i64>,
    ) -> Result<Option<BytesMessage>, Error> {
        unsafe {
            let consumer_destination = SERVER.consumer.clone().unwrap();
            let messages = SERVER.messages.clone();
            for (dest, msg) in messages {
                if dest == consumer_destination {
                    return match msg {
                        Message::BytesMessage(ref msg) => Ok(Some(msg.to_owned())),
                        _ => Ok(None),
                    };
                };
            }
        }
        Ok(None)
    }

    #[cfg(not(feature = "ems-sys"))]
    /// receive map messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// the wait time is ignored
    pub fn receive_map_message(
        &self,
        _wait_time_ms: Option<i64>,
    ) -> Result<Option<MapMessage>, Error> {
        unsafe {
            let consumer_destination = SERVER.consumer.clone().unwrap();
            let messages = SERVER.messages.clone();
            for (dest, msg) in messages {
                if dest == consumer_destination {
                    return match msg {
                        Message::MapMessage(ref msg) => Ok(Some(msg.to_owned())),
                        _ => Ok(None),
                    };
                };
            }
        }
        Ok(None)
    }

    #[cfg(not(feature = "ems-sys"))]
    /// receive object messages from a consumer
    ///
    /// function returns after wait time with a Message or None
    /// the wait time is ignored
    pub fn receive_object_message(
        &self,
        _wait_time_ms: Option<i64>,
    ) -> Result<Option<ObjectMessage>, Error> {
        unsafe {
            let consumer_destination = SERVER.consumer.clone().unwrap();
            let messages = SERVER.messages.clone();
            for (dest, msg) in messages {
                if dest == consumer_destination {
                    return match msg {
                        Message::ObjectMessage(ref msg) => Ok(Some(msg.to_owned())),
                        _ => Ok(None),
                    };
                };
            }
        }
        Ok(None)
    }
}

//
// session
//

impl Session {
    #[cfg(feature = "ems-sys")]
    /// open a message consumer for a queue
    pub fn queue_consumer(
        &self,
        destination: &Destination,
        selector: Option<&str>,
    ) -> Result<Consumer, Error> {
        let consumer: Consumer;
        let mut destination_pointer: usize = 0;
        unsafe {
            //create destination
            match destination {
                Destination::Queue(name) => {
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut destination_pointer,
                        tibemsDestinationType::TIBEMS_QUEUE,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsDestination_Create: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("create destination failed: {}", status_str),
                            ));
                        }
                    }
                }
                Destination::Topic(name) => {
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut destination_pointer,
                        tibemsDestinationType::TIBEMS_TOPIC,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsDestination_Create: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("create destination failed: {}", status_str),
                            ));
                        }
                    }
                }
            }
            //open consumer
            let mut consumer_pointer: usize = 0;
            let c_selector: CString = match selector {
                Some(val) => CString::new(val).unwrap(),
                _ => CString::new("".to_string()).unwrap(),
            };
            let status = tibco_ems_sys::tibemsSession_CreateConsumer(
                self.pointer,
                &mut consumer_pointer,
                destination_pointer,
                c_selector.as_ptr(),
                tibco_ems_sys::tibems_bool::TIBEMS_TRUE,
            );
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateConsumer: {:?}", status),
                _ => {
                    let status_str = format!("{:?}", status);
                    error!("tibemsSession_CreateConsumer: {}", status_str);
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("create consumer failed: {}", status_str),
                    ));
                }
            }
            consumer = Consumer {
                pointer: consumer_pointer,
            };
        }
        Ok(consumer)
    }
    #[cfg(not(feature = "ems-sys"))]
    /// open a message consumer for a queue
    pub fn queue_consumer(
        &self,
        destination: &Destination,
        _selector: Option<&str>,
    ) -> Result<Consumer, Error> {
        unsafe {
            SERVER.consumer = Some(destination.clone());
        }
        Ok(Consumer { pointer: 0 })
    }

    #[cfg(feature = "ems-sys")]
    /// open a message consumer for a topic
    pub fn topic_consumer(
        &self,
        destination: &Destination,
        subscription_name: &str,
        selector: Option<&str>,
    ) -> Result<Consumer, Error> {
        let consumer: Consumer;
        let mut destination_pointer: usize = 0;
        unsafe {
            //create destination
            match destination {
                Destination::Topic(name) => {
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut destination_pointer,
                        tibemsDestinationType::TIBEMS_TOPIC,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsDestination_Create: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("create destination failed: {}", status_str),
                            ));
                        }
                    }
                }
                Destination::Queue(_) => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "destination is not of type topic",
                    ));
                }
            }
            //open consumer
            let mut consumer_pointer: usize = 0;
            let c_subscription_name = CString::new((*subscription_name).to_string()).unwrap();
            let c_selector: CString = match selector {
                Some(val) => CString::new(val).unwrap(),
                _ => CString::new("".to_string()).unwrap(),
            };
            let status = tibco_ems_sys::tibemsSession_CreateSharedConsumer(
                self.pointer,
                &mut consumer_pointer,
                destination_pointer,
                c_subscription_name.as_ptr(),
                c_selector.as_ptr(),
            );
            match status {
                tibems_status::TIBEMS_OK => {
                    trace!("tibemsSession_CreateSharedConsumer: {:?}", status)
                }
                _ => {
                    let status_str = format!("{:?}", status);
                    error!("tibemsSession_CreateSharedConsumer: {}", status_str);
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("create consumer failed: {}", status_str),
                    ));
                }
            }
            consumer = Consumer {
                pointer: consumer_pointer,
            };
        }
        Ok(consumer)
    }

    #[cfg(not(feature = "ems-sys"))]
    /// open a message consumer for a topic
    pub fn topic_consumer(
        &self,
        _destination: &Destination,
        _subscription_name: &str,
        _selector: Option<&str>,
    ) -> Result<Consumer, Error> {
        unimplemented!()
    }

    #[cfg(feature = "ems-sys")]
    /// open a durable message consumer for a topic
    pub fn topic_durable_consumer(
        &self,
        destination: &Destination,
        durable_name: &str,
        selector: Option<&str>,
    ) -> Result<Consumer, Error> {
        let consumer: Consumer;
        let mut destination_pointer: usize = 0;
        unsafe {
            //create destination
            match destination {
                Destination::Topic(name) => {
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut destination_pointer,
                        tibemsDestinationType::TIBEMS_TOPIC,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsDestination_Create: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("create destination failed: {}", status_str),
                            ));
                        }
                    }
                }
                Destination::Queue(_) => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "destination is not of type topic",
                    ));
                }
            }
            //open consumer
            let mut consumer_pointer: usize = 0;
            let c_durable_name = CString::new((*durable_name).to_string()).unwrap();
            let c_selector: CString = match selector {
                Some(val) => CString::new(val).unwrap(),
                _ => CString::new("".to_string()).unwrap(),
            };
            let status = tibco_ems_sys::tibemsSession_CreateSharedDurableConsumer(
                self.pointer,
                &mut consumer_pointer,
                destination_pointer,
                c_durable_name.as_ptr(),
                c_selector.as_ptr(),
            );
            match status {
                tibems_status::TIBEMS_OK => {
                    trace!("tibemsSession_CreateSharedDurableConsumer: {:?}", status)
                }
                _ => {
                    let status_str = format!("{:?}", status);
                    error!("tibemsSession_CreateSharedDurableConsumer: {}", status_str);
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("create consumer failed: {}", status_str),
                    ));
                }
            }
            consumer = Consumer {
                pointer: consumer_pointer,
            };
        }
        Ok(consumer)
    }

    #[cfg(not(feature = "ems-sys"))]
    /// open a durable message consumer for a topic
    pub fn topic_durable_consumer(
        &self,
        _destination: &Destination,
        _durable_name: &str,
        _selector: Option<&str>,
    ) -> Result<Consumer, Error> {
        unimplemented!()
    }

    #[cfg(feature = "ems-sys")]
    /// close a session
    fn close(&self) {
        unsafe {
            //destroy producer
            if self.producer_pointer != 0 {
                let status = tibco_ems_sys::tibemsMsgProducer_Close(self.producer_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Close: {:?}", status),
                    _ => error!("tibemsMsgProducer_Close: {:?}", status),
                }
            }
            let status = tibco_ems_sys::tibemsSession_Close(self.pointer);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsSession_Close: {:?}", status),
                _ => error!("tibemsSession_Close: {:?}", status),
            }
        }
    }
    #[cfg(not(feature = "ems-sys"))]
    /// close a session
    fn close(&self) {}

    #[cfg(feature = "tracing")]
    fn add_trace_to_message(&self, message: &mut Message) -> impl opentelemetry::trace::Span {
        let tracer_provider = opentelemetry::global::tracer_provider();
        use opentelemetry::sdk::trace::IdGenerator;
        use opentelemetry::sdk::trace::RandomIdGenerator;
        use opentelemetry::trace::Span;
        use opentelemetry::trace::SpanId;
        use opentelemetry::trace::TraceId;
        use opentelemetry::trace::Tracer;
        use opentelemetry::trace::TracerProvider;
        let tracer = tracer_provider.versioned_tracer("ems", Some("0.5"), None);
        let span = tracer.start("send");
        let id_generator = RandomIdGenerator::default();
        let headers = match message {
            Message::BytesMessage(b) => b.header.as_mut(),
            Message::MapMessage(m) => m.header.as_mut(),
            Message::TextMessage(t) => t.header.as_mut(),
            Message::ObjectMessage(o) => o.header.as_mut(),
        };
        let ctx = span.span_context();
        let span_id = if ctx.span_id() == SpanId::INVALID {
            id_generator.new_span_id()
        } else {
            ctx.span_id()
        };
        let trace_id = if ctx.trace_id() == TraceId::INVALID {
            id_generator.new_trace_id()
        } else {
            ctx.trace_id()
        };
        if let Some(e) = headers {
            e.insert(
                "spanId".to_string(),
                TypedValue::String(span_id.to_string()),
            );
            e.insert(
                "traceId".to_string(),
                TypedValue::String(trace_id.to_string()),
            );
        };
        span
    }

    #[cfg(feature = "ems-sys")]
    /// sending a message to a destination (only queues are supported)
    pub fn send_message<M: Into<Message>>(
        &self,
        destination: &Destination,
        message: M,
    ) -> Result<(), Error> {
        #[cfg(feature = "tracing")]
        let mut message: Message = message.into();
        #[cfg(not(feature = "tracing"))]
        let message: Message = message.into();

        let mut dest: usize = 0;
        let mut local_producer: usize = 0;
        #[cfg(feature = "tracing")]
        let mut span = self.add_trace_to_message(&mut message);
        #[cfg(feature = "tracing")]
        use opentelemetry::trace::Span;
        unsafe {
            match destination {
                Destination::Queue(name) => {
                    #[cfg(feature = "tracing")]
                    span.set_attribute(opentelemetry::KeyValue::new(
                        "messaging.destination",
                        name.clone(),
                    ));
                    #[cfg(feature = "tracing")]
                    span.set_attribute(opentelemetry::KeyValue::new(
                        "messaging.destination_kind",
                        "queue",
                    ));
                    #[cfg(feature = "tracing")]
                    span.set_attribute(opentelemetry::KeyValue::new(
                        "messaging.system",
                        "TibcoEMS",
                    ));
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut dest,
                        tibemsDestinationType::TIBEMS_QUEUE,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsDestination_Create: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("create destination failed: {}", status_str),
                            ));
                        }
                    }
                }
                Destination::Topic(name) => {
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut dest,
                        tibemsDestinationType::TIBEMS_TOPIC,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => {
                            let status_str = format!("{:?}", status);
                            error!("tibemsDestination_Create: {}", status_str);
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("create destination failed: {}", status_str),
                            ));
                        }
                    }
                }
            }
            if self.producer_pointer == 0 {
                let status = tibco_ems_sys::tibemsSession_CreateProducer(
                    self.pointer,
                    &mut local_producer,
                    dest,
                );
                match status {
                    tibems_status::TIBEMS_OK => {
                        trace!("tibemsSession_CreateProducer: {:?}", status)
                    }
                    _ => {
                        let status_str = format!("{:?}", status);
                        error!("tibemsSession_CreateProducer: {}", status_str);
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("create producer failed: {}", status_str),
                        ));
                    }
                }
            }
            let msg = build_message_pointer_from_message(&message);
            let status = tibco_ems_sys::tibemsMsgProducer_SendToDestination(
                self.producer_pointer,
                dest,
                msg,
            );
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Send: {:?}", status),
                _ => {
                    let status_str = format!("{:?}", status);
                    error!("tibemsMsgProducer_Send: {}", status_str);
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("send message failed: {}", status_str),
                    ));
                }
            }
            //destroy producer if generated inline
            if self.producer_pointer == 0 {
                let status = tibco_ems_sys::tibemsMsgProducer_Close(local_producer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Close: {:?}", status),
                    _ => error!("tibemsMsgProducer_Close: {:?}", status),
                }
            }
            //destroy message
            let status = tibco_ems_sys::tibemsMsg_Destroy(msg);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}", status),
                _ => error!("tibemsMsg_Destroy: {:?}", status),
            }
            //destroy destination
            let status = tibco_ems_sys::tibemsDestination_Destroy(dest);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsDestination_Destroy: {:?}", status),
                _ => error!("tibemsDestination_Destroy: {:?}", status),
            }
        }
        #[cfg(feature = "tracing")]
        span.end();
        Ok(())
    }

    #[cfg(not(feature = "ems-sys"))]
    /// sending a message to a destination (only queues are supported)
    pub fn send_message<M: Into<Message>>(
        &self,
        destination: &Destination,
        message: M,
    ) -> Result<(), Error> {
        let message: Message = message.into();
        unsafe {
            SERVER.messages.push((destination.clone(), message));
        }
        Ok(())
    }

    #[cfg(feature = "ems-sys")]
    /// request/reply
    pub fn request_reply<M: Into<Message>>(
        &self,
        destination: &Destination,
        message: M,
        timeout: i64,
    ) -> Result<Option<Message>, Error> {
        let message: Message = message.into();
        //create temporary destination
        let mut reply_dest: usize = 0;
        let mut dest: usize = 0;
        unsafe {
            match &destination {
                Destination::Queue(name) => {
                    let status = tibco_ems_sys::tibemsSession_CreateTemporaryQueue(
                        self.pointer,
                        &mut reply_dest,
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsSession_CreateTemporaryQueue: {:?}", status)
                        }
                        _ => error!("tibemsSession_CreateTemporaryQueue: {:?}", status),
                    }
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut dest,
                        tibemsDestinationType::TIBEMS_QUEUE,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => error!("tibemsDestination_Create: {:?}", status),
                    }
                }
                Destination::Topic(name) => {
                    let status = tibco_ems_sys::tibemsSession_CreateTemporaryTopic(
                        self.pointer,
                        &mut reply_dest,
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsSession_CreateTemporaryTopic: {:?}", status)
                        }
                        _ => error!("tibemsSession_CreateTemporaryTopic: {:?}", status),
                    }
                    let c_destination = CString::new(name.clone()).unwrap();
                    let status = tibco_ems_sys::tibemsDestination_Create(
                        &mut dest,
                        tibemsDestinationType::TIBEMS_TOPIC,
                        c_destination.as_ptr(),
                    );
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsDestination_Create: {:?}", status)
                        }
                        _ => error!("tibemsDestination_Create: {:?}", status),
                    }
                }
            }
            let mut producer: usize = 0;
            let status =
                tibco_ems_sys::tibemsSession_CreateProducer(self.pointer, &mut producer, dest);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}", status),
                _ => error!("tibemsSession_CreateProducer: {:?}", status),
            }
            let msg = build_message_pointer_from_message(&message);
            //set reply to
            let status = tibco_ems_sys::tibemsMsg_SetReplyTo(msg, reply_dest);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetReplyTo: {:?}", status),
                _ => error!("tibemsMsg_SetReplyTo: {:?}", status),
            }
            let status = tibco_ems_sys::tibemsMsgProducer_Send(producer, msg);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Send: {:?}", status),
                _ => error!("tibemsMsgProducer_Send: {:?}", status),
            }
            //destroy message
            let status = tibco_ems_sys::tibemsMsg_Destroy(msg);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}", status),
                _ => error!("tibemsMsg_Destroy: {:?}", status),
            }
            //destroy producer
            let status = tibco_ems_sys::tibemsMsgProducer_Close(producer);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Close: {:?}", status),
                _ => error!("tibemsMsgProducer_Close: {:?}", status),
            }
            //destroy destination
            let status = tibco_ems_sys::tibemsDestination_Destroy(dest);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsDestination_Destroy: {:?}", status),
                _ => error!("tibemsDestination_Destroy: {:?}", status),
            }
            //open consumer
            let mut consumer_pointer: usize = 0;
            let status = tibco_ems_sys::tibemsSession_CreateConsumer(
                self.pointer,
                &mut consumer_pointer,
                reply_dest,
                std::ptr::null(),
                tibco_ems_sys::tibems_bool::TIBEMS_TRUE,
            );
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateConsumer: {:?}", status),
                _ => error!("tibemsSession_CreateConsumer: {:?}", status),
            }
            let mut reply_message: usize = 0;
            let status = tibco_ems_sys::tibemsMsgConsumer_ReceiveTimeout(
                consumer_pointer,
                &mut reply_message,
                timeout,
            );
            match status {
                tibems_status::TIBEMS_OK => {
                    trace!("tibemsMsgConsumer_ReceiveTimeout: {:?}", status)
                }
                tibems_status::TIBEMS_TIMEOUT => {
                    return Ok(None);
                }
                _ => error!("tibemsMsgConsumer_ReceiveTimeout: {:?}", status),
            }
            let result = build_message_from_pointer(reply_message);
            //close consumer
            let status = tibco_ems_sys::tibemsMsgConsumer_Close(consumer_pointer);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_Close: {:?}", status),
                _ => error!("tibemsMsgConsumer_Close: {:?}", status),
            }
            //destroy temporary destination
            match &destination {
                Destination::Queue { .. } => {
                    //destroy reply_to_queue
                    let status =
                        tibco_ems_sys::tibemsSession_DeleteTemporaryQueue(self.pointer, reply_dest);
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsSession_DeleteTemporaryQueue: {:?}", status)
                        }
                        _ => error!("tibemsSession_DeleteTemporaryQueue: {:?}", status),
                    }
                }
                Destination::Topic { .. } => {
                    //destroy reply_to_queue
                    let status =
                        tibco_ems_sys::tibemsSession_DeleteTemporaryTopic(self.pointer, reply_dest);
                    match status {
                        tibems_status::TIBEMS_OK => {
                            trace!("tibemsSession_DeleteTemporaryTopic: {:?}", status)
                        }
                        _ => error!("tibemsSession_DeleteTemporaryTopic: {:?}", status),
                    }
                }
            }
            Ok(Some(result))
        }
    }

    #[cfg(not(feature = "ems-sys"))]
    /// request/reply - always returns none
    pub fn request_reply<M: Into<Message>>(
        &self,
        destination: &Destination,
        message: M,
        _timeout: i64,
    ) -> Result<Option<Message>, Error> {
        let message: Message = message.into();
        unsafe {
            SERVER.messages.push((destination.clone(), message));
        }
        Ok(None)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.close();
    }
}

//
// messages
//

impl From<MapMessage> for Message {
    fn from(msg: MapMessage) -> Self {
        Message::MapMessage(msg)
    }
}

impl From<TextMessage> for Message {
    fn from(msg: TextMessage) -> Self {
        Message::TextMessage(msg)
    }
}

impl From<BytesMessage> for Message {
    fn from(msg: BytesMessage) -> Self {
        Message::BytesMessage(msg)
    }
}

impl From<ObjectMessage> for Message {
    fn from(msg: ObjectMessage) -> Self {
        Message::ObjectMessage(msg)
    }
}

/// represents a typed value, which is used for message header and message properties
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypedValue {
    /// represents a String Value
    String(String),
    /// represents a integer Value
    Integer(i32),
    /// represents a long Value
    Long(i64),
    /// represents a float Value
    Float(f32),
    /// represents a double Value
    Double(f64),
    /// represents a binary Value
    Binary(Vec<u8>),
    /// represents a MapMessage Value
    Map(MapMessage),
    /// represents a boolean Value
    Boolean(bool),
}

impl fmt::Display for TypedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypedValue::String(s) => write!(f, "{s}"),
            TypedValue::Boolean(b) => write!(f, "{b}"),
            TypedValue::Integer(b) => write!(f, "{b}"),
            TypedValue::Long(b) => write!(f, "{b}"),
            TypedValue::Float(b) => write!(f, "{b}"),
            TypedValue::Double(b) => write!(f, "{b}"),
            TypedValue::Binary(b) => write!(f, "{b:?}"),
            TypedValue::Map(m) => write!(f, "{m:?}"),
        }
    }
}

impl Message {
    #[cfg(feature = "ems-sys")]
    fn destroy(&self) {
        let destroy_msg = |pointer: usize| unsafe {
            let status = tibco_ems_sys::tibemsMsg_Destroy(pointer);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}", status),
                _ => error!("tibemsMsg_Destroy: {:?}", status),
            }
        };
        match self {
            Message::TextMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    destroy_msg(pointer);
                }
            }
            Message::BytesMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    destroy_msg(pointer);
                }
            }
            Message::ObjectMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    destroy_msg(pointer);
                }
            }
            Message::MapMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    destroy_msg(pointer);
                }
            }
        }
    }

    #[cfg(not(feature = "ems-sys"))]
    fn destroy(&self) {}

    #[cfg(feature = "ems-sys")]
    /// confirms the message by invoking tibemsMsg_Acknowledge
    pub fn confirm(&self) {
        let ack_msg = |pointer: usize| unsafe {
            let status = tibco_ems_sys::tibemsMsg_Acknowledge(pointer);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_Acknowledge: {:?}", status),
                _ => error!("tibemsMsg_Acknowledge: {:?}", status),
            }
        };
        match self {
            Message::TextMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    ack_msg(pointer);
                }
            }
            Message::BytesMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    ack_msg(pointer);
                }
            }
            Message::ObjectMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    ack_msg(pointer);
                }
            }
            Message::MapMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    ack_msg(pointer);
                }
            }
        }
    }
    #[cfg(not(feature = "ems-sys"))]
    /// confirms the message by invoking tibemsMsg_Acknowledge
    pub fn confirm(&self) {}

    #[cfg(feature = "ems-sys")]
    /// rolls the message back by invoking tibemsMsg_Recover
    pub fn rollback(&self) {
        let recover = |pointer: usize| unsafe {
            let status = tibco_ems_sys::tibemsMsg_Recover(pointer);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_Recover: {:?}", status),
                _ => error!("tibemsMsg_Recover: {:?}", status),
            }
        };
        match self {
            Message::TextMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    recover(pointer);
                }
            }
            Message::BytesMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    recover(pointer);
                }
            }
            Message::ObjectMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    recover(pointer);
                }
            }
            Message::MapMessage(msg) => {
                if let Some(pointer) = msg.pointer {
                    recover(pointer);
                }
            }
        }
    }

    #[cfg(not(feature = "ems-sys"))]
    /// rolls the message back by invoking tibemsMsg_Recover
    pub fn rollback(&self) {}
}

impl Drop for Message {
    fn drop(&mut self) {
        self.destroy();
    }
}

#[cfg(feature = "ems-sys")]
fn build_message_pointer_from_message(message: &Message) -> usize {
    let mut msg_pointer: usize = 0;
    unsafe {
        match message {
            Message::TextMessage(msg) => {
                let status = tibco_ems_sys::tibemsTextMsg_Create(&mut msg_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_Create: {:?}", status),
                    _ => error!("tibemsTextMsg_Create: {:?}", status),
                }
                let c_text = CString::new(msg.body.clone()).unwrap();
                let status = tibco_ems_sys::tibemsTextMsg_SetText(msg_pointer, c_text.as_ptr());
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_SetText: {:?}", status),
                    _ => error!("tibemsTextMsg_SetText: {:?}", status),
                }
            }
            Message::BytesMessage(msg) => {
                let status = tibco_ems_sys::tibemsBytesMsg_Create(&mut msg_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsBytesMsg_Create: {:?}", status),
                    _ => error!("tibemsBytesMsg_Create: {:?}", status),
                }
                let content = msg.body.clone();
                let body_size = content.len();
                if body_size > 0 {
                    let body_ptr = content.as_ptr() as *const c_void;
                    let status = tibco_ems_sys::tibemsBytesMsg_SetBytes(
                        msg_pointer,
                        body_ptr,
                        body_size as u32,
                    );
                    match status {
                        tibems_status::TIBEMS_OK => trace!("tibemsBytesMsg_SetBytes: {:?}", status),
                        _ => error!("tibemsBytesMsg_SetBytes: {:?}", status),
                    }
                }
            }
            Message::ObjectMessage(msg) => {
                let status = tibco_ems_sys::tibemsObjectMsg_Create(&mut msg_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsObjectMsg_Create: {:?}", status),
                    _ => error!("tibemsObjectMsg_Create: {:?}", status),
                }
                let content = msg.body.clone();
                let body_size = content.len();
                let body_ptr = content.as_ptr() as *const c_void;
                let status = tibco_ems_sys::tibemsObjectMsg_SetObjectBytes(
                    msg_pointer,
                    body_ptr,
                    body_size as u32,
                );
                match status {
                    tibems_status::TIBEMS_OK => {
                        trace!("tibemsObjectMsg_SetObjectBytes: {:?}", status)
                    }
                    _ => error!("tibemsObjectMsg_SetObjectBytes: {:?}", status),
                }
            }
            Message::MapMessage(msg) => {
                let status = tibco_ems_sys::tibemsMapMsg_Create(&mut msg_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_Create: {:?}", status),
                    _ => error!("tibemsMapMsg_Create: {:?}", status),
                }
                for (key, val) in msg.body.clone() {
                    let c_name = CString::new(key).unwrap();
                    match val {
                        TypedValue::Boolean(value) => {
                            let status = if value {
                                tibco_ems_sys::tibemsMapMsg_SetBoolean(
                                    msg_pointer,
                                    c_name.as_ptr(),
                                    tibems_bool::TIBEMS_TRUE,
                                )
                            } else {
                                tibco_ems_sys::tibemsMapMsg_SetBoolean(
                                    msg_pointer,
                                    c_name.as_ptr(),
                                    tibems_bool::TIBEMS_FALSE,
                                )
                            };
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_SetBoolean: {:?}", status)
                                }
                                _ => error!("tibemsMapMsg_SetBoolean: {:?}", status),
                            }
                        }
                        TypedValue::String(value) => {
                            let c_value = CString::new(value).unwrap();
                            let status = tibco_ems_sys::tibemsMapMsg_SetString(
                                msg_pointer,
                                c_name.as_ptr(),
                                c_value.as_ptr(),
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_SetString: {:?}", status)
                                }
                                _ => error!("tibemsMapMsg_SetString: {:?}", status),
                            }
                        }
                        TypedValue::Integer(value) => {
                            let status = tibco_ems_sys::tibemsMapMsg_SetInt(
                                msg_pointer,
                                c_name.as_ptr(),
                                value,
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_SetInt: {:?}", status)
                                }
                                _ => error!("tibemsMapMsg_SetInt: {:?}", status),
                            }
                        }
                        TypedValue::Long(value) => {
                            let status = tibco_ems_sys::tibemsMapMsg_SetLong(
                                msg_pointer,
                                c_name.as_ptr(),
                                value,
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_SetLong: {:?}", status)
                                }
                                _ => error!("tibemsMapMsg_SetLong: {:?}", status),
                            }
                        }
                        TypedValue::Float(value) => {
                            let status = tibco_ems_sys::tibemsMapMsg_SetFloat(
                                msg_pointer,
                                c_name.as_ptr(),
                                value,
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_SetFloat: {:?}", status)
                                }
                                _ => error!("tibemsMapMsg_SetFloat: {:?}", status),
                            }
                        }

                        TypedValue::Double(value) => {
                            let status = tibco_ems_sys::tibemsMapMsg_SetDouble(
                                msg_pointer,
                                c_name.as_ptr(),
                                value,
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_SetDouble: {:?}", status)
                                }
                                _ => error!("tibemsMapMsg_SetDouble: {:?}", status),
                            }
                        }
                        TypedValue::Binary(_value) => {
                            //TODO implement
                            // let status = tibco_ems_sys::tibemsMapMsg_SetBytes(message: usize, name: *const c_char, bytes: *mut c_void, bytesSize: u64)Long(msg, c_name.as_ptr(), value);
                            // match status {
                            // tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetLong: {:?}",status),
                            // _ => error!("tibemsMapMsg_SetLong: {:?}",status),
                            // }
                        }
                        _ => {
                            panic!("missing map message type implementation for {:?}", val);
                        }
                    }
                }
            }
        }
        //set header
        let header = match message {
            Message::TextMessage(msg) => msg.header.clone(),
            Message::BytesMessage(msg) => msg.header.clone(),
            Message::MapMessage(msg) => msg.header.clone(),
            Message::ObjectMessage(msg) => msg.header.clone(),
        };
        if let Some(headers) = header {
            //look for correlation id
            if let Some(correlation_id) = headers.get("CorrelationID") {
                let correlation_id_val = extract!(TypedValue::String(_), correlation_id)
                    .expect("extract correlation id");
                let c_correlation_id = CString::new(correlation_id_val.as_str()).unwrap();
                let status = tibco_ems_sys::tibemsMsg_SetCorrelationID(
                    msg_pointer,
                    c_correlation_id.as_ptr(),
                );
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetCorrelationId: {:?}", status),
                    _ => error!("tibemsMsg_SetCorrelationId: {:?}", status),
                }
            }
            //look for jms type
            if let Some(jms_type) = headers.get("JMSType") {
                let jms_type_val =
                    extract!(TypedValue::String(_), jms_type).expect("extract correlation id");
                let c_jms_type = CString::new(jms_type_val.as_str()).unwrap();
                let status = tibco_ems_sys::tibemsMsg_SetType(msg_pointer, c_jms_type.as_ptr());
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetType: {:?}", status),
                    _ => error!("tibemsMsg_SetType: {:?}", status),
                }
            }
            //do other headers (also do correlation id again as custom header)
            for (key, val) in &headers {
                let c_name = CString::new(key.to_string()).unwrap();
                match val {
                    TypedValue::String(value) => {
                        let c_val = CString::new(value.as_bytes()).unwrap();
                        let status = tibco_ems_sys::tibemsMsg_SetStringProperty(
                            msg_pointer,
                            c_name.as_ptr(),
                            c_val.as_ptr(),
                        );
                        match status {
                            tibems_status::TIBEMS_OK => {
                                trace!("tibemsMsg_SetStringProperty: {:?}", status)
                            }
                            _ => error!("tibemsMsg_SetStringProperty: {:?}", status),
                        }
                    }
                    TypedValue::Boolean(value) => {
                        let status = if *value {
                            tibco_ems_sys::tibemsMsg_SetBooleanProperty(
                                msg_pointer,
                                c_name.as_ptr(),
                                tibems_bool::TIBEMS_TRUE,
                            )
                        } else {
                            tibco_ems_sys::tibemsMsg_SetBooleanProperty(
                                msg_pointer,
                                c_name.as_ptr(),
                                tibems_bool::TIBEMS_FALSE,
                            )
                        };
                        match status {
                            tibems_status::TIBEMS_OK => {
                                trace!("tibemsMsg_SetBooleanProperty: {:?}", status)
                            }
                            _ => error!("tibemsMsg_SetBooleanProperty: {:?}", status),
                        }
                    }
                    TypedValue::Integer(value) => {
                        let status = tibco_ems_sys::tibemsMsg_SetIntProperty(
                            msg_pointer,
                            c_name.as_ptr(),
                            *value,
                        );
                        match status {
                            tibems_status::TIBEMS_OK => {
                                trace!("tibemsMsg_SetIntProperty: {:?}", status)
                            }
                            _ => error!("tibemsMsg_SetIntProperty: {:?}", status),
                        }
                    }
                    TypedValue::Long(value) => {
                        let status = tibco_ems_sys::tibemsMsg_SetLongProperty(
                            msg_pointer,
                            c_name.as_ptr(),
                            *value,
                        );
                        match status {
                            tibems_status::TIBEMS_OK => {
                                trace!("tibemsMsg_SetLongProperty: {:?}", status)
                            }
                            _ => error!("tibemsMsg_SetLongProperty: {:?}", status),
                        }
                    }
                    _ => {
                        panic!("missing property type implementation for {:?}", val);
                    }
                }
            }
        }
    }
    msg_pointer
}

#[cfg(feature = "ems-sys")]
fn build_message_from_pointer(msg_pointer: usize) -> Message {
    let mut msg: Message;
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    unsafe {
        let mut msg_type: tibemsMsgType = tibemsMsgType::TIBEMS_TEXT_MESSAGE;
        let status = tibco_ems_sys::tibemsMsg_GetBodyType(msg_pointer, &mut msg_type);
        match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetBodyType: {:?}", status),
            _ => error!("tibemsMsg_GetBodyType: {:?}", status),
        }
        match msg_type {
            tibemsMsgType::TIBEMS_TEXT_MESSAGE => {
                let buf_vec: Vec<i8> = vec![0; 0];
                let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
                let status = tibco_ems_sys::tibemsTextMsg_GetText(msg_pointer, &buf_ref);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_GetText: {:?}", status),
                    _ => error!("tibemsTextMsg_GetText: {:?}", status),
                }
                let content = CStr::from_ptr(buf_ref).to_str().unwrap();
                let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}", status),
                    _ => error!("tibemsMsg_GetMessageID: {:?}", status),
                }
                let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
                header.insert(
                    "MessageID".to_string(),
                    TypedValue::String(message_id.to_string()),
                );
                msg = Message::TextMessage(TextMessage {
                    body: content.to_string(),
                    header: None,
                    pointer: Some(msg_pointer),
                    destination: None,
                    reply_to: None,
                });
            }
            tibemsMsgType::TIBEMS_MAP_MESSAGE => {
                let buf_vec: Vec<i8> = vec![0; 0];
                let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
                let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}", status),
                    _ => error!("tibemsMsg_GetMessageID: {:?}", status),
                }
                //admin messages do not have a message id
                if !buf_vec.is_empty() {
                    let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
                    header.insert(
                        "MessageID".to_string(),
                        TypedValue::String(message_id.to_string()),
                    );
                }
                let mut names_pointer: usize = 0;
                trace!("tibemsMapMsg_GetMapNames");
                let status =
                    tibco_ems_sys::tibemsMapMsg_GetMapNames(msg_pointer, &mut names_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_GetMapNames: {:?}", status),
                    _ => error!("tibemsMapMsg_GetMapNames: {:?}", status),
                }
                let mut body_entries: HashMap<String, TypedValue> = HashMap::new();
                loop {
                    let buf_vec: Vec<i8> = vec![0; 0];
                    let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
                    let status = tibco_ems_sys::tibemsMsgEnum_GetNextName(names_pointer, &buf_ref);
                    match status {
                        tibems_status::TIBEMS_OK => {
                            let header_name = CStr::from_ptr(buf_ref).to_str().unwrap();
                            trace!("getting value for property: {}", header_name);
                            let mut val_buf_vec: Vec<i8> = vec![0; 0];
                            let mut val_buf_ref: *mut std::os::raw::c_char =
                                val_buf_vec.as_mut_ptr();
                            let status = tibco_ems_sys::tibemsMapMsg_GetString(
                                msg_pointer,
                                buf_ref,
                                &mut val_buf_ref,
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsMapMsg_GetString: {:?}", status);
                                    if !val_buf_ref.is_null() {
                                        let header_value =
                                            CStr::from_ptr(val_buf_ref).to_str().unwrap();
                                        body_entries.insert(
                                            header_name.to_string(),
                                            TypedValue::String(header_value.to_string()),
                                        );
                                    }
                                }
                                tibems_status::TIBEMS_CONVERSION_FAILED => {
                                    //it must be a map msg inside
                                    let mut msg2: usize = 0;
                                    let status = tibco_ems_sys::tibemsMapMsg_GetMapMsg(
                                        msg_pointer,
                                        buf_ref,
                                        &mut msg2,
                                    );
                                    match status {
                                        tibems_status::TIBEMS_CONVERSION_FAILED => {
                                            //it must be something binary, ingore it for now
                                            trace!(
                                                "tibemsMapMsg_GetMapMsg: ignoring unkown content"
                                            );
                                        }
                                        tibems_status::TIBEMS_OK => {
                                            trace!("tibemsMapMsg_GetMapMsg: {:?}", status);
                                            let mut raw_message = build_message_from_pointer(msg2);
                                            match &mut raw_message {
                                                Message::TextMessage(_msg) => {}
                                                Message::ObjectMessage(_msg) => {}
                                                Message::BytesMessage(_msg) => {}
                                                Message::MapMessage(msg) => {
                                                    msg.pointer = None;
                                                    body_entries.insert(
                                                        header_name.to_string(),
                                                        TypedValue::Map(msg.clone()),
                                                    );
                                                }
                                            }
                                        }
                                        _ => error!("tibemsMapMsg_GetMapMsg: {:?}", status),
                                    }
                                }
                                _ => error!("tibemsMapMsg_GetString: {:?}", status),
                            }
                        }
                        tibems_status::TIBEMS_NOT_FOUND => {
                            break;
                        }
                        _ => {
                            println!("tibemsMsgEnum_GetNextName: {:?}", status);
                            break;
                        }
                    }
                }
                let status = tibco_ems_sys::tibemsMsgEnum_Destroy(names_pointer);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsgEnum_Destroy: {:?}", status),
                    _ => error!("tibemsMsgEnum_Destroy: {:?}", status),
                }
                msg = Message::MapMessage(MapMessage {
                    body: body_entries,
                    header: None,
                    pointer: Some(msg_pointer),
                    destination: None,
                    reply_to: None,
                });
            }
            tibemsMsgType::TIBEMS_BYTES_MESSAGE => {
                let buf_vec: Vec<i8> = vec![0; 0];
                let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
                let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}", status),
                    _ => error!("tibemsMsg_GetMessageID: {:?}", status),
                }
                let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
                header.insert(
                    "MessageID".to_string(),
                    TypedValue::String(message_id.to_string()),
                );
                //check body length
                let mut body_length: i32 = 0;
                let mut body_value: Vec<u8> = vec![0; 0];
                let status =
                    tibco_ems_sys::tibemsBytesMsg_GetBodyLength(msg_pointer, &mut body_length);
                match status {
                    tibems_status::TIBEMS_OK => {
                        trace!("tibemsBytesMsg_GetBodyLength: {:?}", status);
                        if body_length > 0 {
                            //extract body
                            let buf_vec: Vec<u8> = vec![0; 0];
                            let buf_ref: *const std::os::raw::c_uchar = buf_vec.as_ptr();
                            let mut result_size: u32 = 0;
                            let status = tibco_ems_sys::tibemsBytesMsg_GetBytes(
                                msg_pointer,
                                &buf_ref,
                                &mut result_size,
                            );
                            match status {
                                tibems_status::TIBEMS_OK => {
                                    trace!("tibemsBytesMsg_GetBytes: {:?}", status)
                                }
                                _ => error!("tibemsBytesMsg_GetBytes: {:?}", status),
                            }
                            let slice = core::slice::from_raw_parts(buf_ref, result_size as usize);
                            body_value = slice.to_vec();
                        }
                    }
                    _ => error!("tibemsBytesMsg_GetBodyLength: {:?}", status),
                }
                msg = Message::BytesMessage(BytesMessage {
                    body: body_value,
                    header: None,
                    pointer: Some(msg_pointer),
                    destination: None,
                    reply_to: None,
                });
            }
            tibemsMsgType::TIBEMS_OBJECT_MESSAGE => {
                let buf_vec: Vec<i8> = vec![0; 0];
                let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
                let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
                match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}", status),
                    _ => error!("tibemsMsg_GetMessageID: {:?}", status),
                }
                let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
                header.insert(
                    "MessageID".to_string(),
                    TypedValue::String(message_id.to_string()),
                );
                //extract body
                let buf_vec: Vec<u8> = vec![0; 0];
                let buf_ref: *const std::os::raw::c_uchar = buf_vec.as_ptr();
                let mut result_size: u32 = 0;
                let status = tibco_ems_sys::tibemsObjectMsg_GetObjectBytes(
                    msg_pointer,
                    &buf_ref,
                    &mut result_size,
                );
                match status {
                    tibems_status::TIBEMS_OK => {
                        trace!("tibemsObjectMsg_GetObjectBytes: {:?}", status)
                    }
                    _ => error!("tibemsObjectMsg_GetObjectBytes: {:?}", status),
                }
                let slice = core::slice::from_raw_parts(buf_ref, result_size as usize);
                msg = Message::ObjectMessage(ObjectMessage {
                    body: slice.to_vec(),
                    header: None,
                    pointer: Some(msg_pointer),
                    destination: None,
                    reply_to: None,
                });
            }
            _ => {
                //unknown
                panic!("BodyType {:?} not implemented", msg_type);
            }
        }
        //add correlation id to header
        let buf_vec: Vec<i8> = vec![0; 0];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsMsg_GetCorrelationID(msg_pointer, &buf_ref);
        match status {
            tibems_status::TIBEMS_OK => {
                trace!("tibemsMsg_GetCorrelationID: {:?}", status);
                // check for null pointer (when no correlation id was set)
                if !buf_ref.is_null() {
                    let correlation_id = CStr::from_ptr(buf_ref).to_str().unwrap();
                    header.insert(
                        "CorrelationID".to_string(),
                        TypedValue::String(correlation_id.to_string()),
                    );
                }
            }
            _ => trace!("tibemsMsg_GetCorrelationID: {:?}", status),
        }
        // fetch header
        let mut header_enumeration: usize = 0;
        let status =
            tibco_ems_sys::tibemsMsg_GetPropertyNames(msg_pointer, &mut header_enumeration);
        match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetPropertyNames: {:?}", status),
            _ => error!("tibemsMsg_GetPropertyNames: {:?}", status),
        }
        loop {
            let buf_vec: Vec<i8> = vec![0; 0];
            let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
            let status = tibco_ems_sys::tibemsMsgEnum_GetNextName(header_enumeration, &buf_ref);
            match status {
                tibems_status::TIBEMS_OK => {
                    let header_name = CStr::from_ptr(buf_ref).to_str().unwrap();
                    let val_buf_vec: Vec<i8> = vec![0; 0];
                    let val_buf_ref: *const std::os::raw::c_char = val_buf_vec.as_ptr();
                    let mut bool_result: tibems_bool = tibems_bool::TIBEMS_TRUE;
                    //check for ems compress header
                    if header_name == "JMS_TIBCO_COMPRESS" {
                        let status = tibco_ems_sys::tibemsMsg_GetBooleanProperty(
                            msg_pointer,
                            buf_ref,
                            &mut bool_result,
                        );
                        match status {
                            tibems_status::TIBEMS_OK => {
                                trace!("tibemsMsg_GetBooleanProperty: {:?}", status);
                                let value = match bool_result {
                                    tibems_bool::TIBEMS_TRUE => true,
                                    tibems_bool::TIBEMS_FALSE => false,
                                };
                                header.insert(header_name.to_string(), TypedValue::Boolean(value));
                            }
                            _ => error!("tibemsMsg_GetBooleanProperty: {:?}", status),
                        }
                    } else {
                        let status = tibco_ems_sys::tibemsMsg_GetStringProperty(
                            msg_pointer,
                            buf_ref,
                            &val_buf_ref,
                        );
                        match status {
                            tibems_status::TIBEMS_OK => {
                                trace!("tibemsMsg_GetStringProperty: {:?}", status)
                            }
                            _ => error!("tibemsMsg_GetStringProperty: {:?}", status),
                        }
                        let header_value = CStr::from_ptr(val_buf_ref).to_str().unwrap();
                        header.insert(
                            header_name.to_string(),
                            TypedValue::String(header_value.to_string()),
                        );
                    }
                }
                tibems_status::TIBEMS_NOT_FOUND => {
                    break;
                }
                _ => {
                    println!("tibemsMsgEnum_GetNextName: {:?}", status);
                    break;
                }
            }
        }
        let status = tibco_ems_sys::tibemsMsgEnum_Destroy(header_enumeration);
        match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsgEnum_Destroy: {:?}", status),
            _ => error!("tibemsMsgEnum_Destroy: {:?}", status),
        }
        //add JMSType to header
        let val_buf_vec: Vec<i8> = vec![0; 0];
        let val_buf_ref: *const std::os::raw::c_char = val_buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsMsg_GetType(msg_pointer, &val_buf_ref);
        match status {
            tibems_status::TIBEMS_OK => {
                trace!("tibemsMsg_GetType: {:?}", status);
                // check for null pointer (when no correlation id was set)
                if !val_buf_ref.is_null() {
                    let header_value = CStr::from_ptr(val_buf_ref).to_str().unwrap();
                    if !header_value.is_empty() {
                        header.insert(
                            "JMSType".to_string(),
                            TypedValue::String(header_value.to_string()),
                        );
                    }
                }
            }
            _ => error!("tibemsMsg_GetType: {:?}", status),
        }
        //add header to message
        match &mut msg {
            Message::TextMessage(msg) => msg.header = Some(header),
            Message::BytesMessage(msg) => msg.header = Some(header),
            Message::MapMessage(msg) => msg.header = Some(header),
            Message::ObjectMessage(msg) => msg.header = Some(header),
        }
        // look for JMSDestination header
        let mut jms_destination: usize = 0;
        let status = tibco_ems_sys::tibemsMsg_GetDestination(msg_pointer, &mut jms_destination);
        match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetDestination: {:?}", status),
            _ => error!("tibemsMsg_GetDestination: {:?}", status),
        }
        if jms_destination != 0 {
            //has a destination
            let mut destination_type = tibemsDestinationType::TIBEMS_UNKNOWN;
            let status =
                tibco_ems_sys::tibemsDestination_GetType(jms_destination, &mut destination_type);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetType: {:?}", status),
                _ => error!("tibemsDestination_GetType: {:?}", status),
            }
            let buf_size = 1024;
            let buf_vec: Vec<i8> = vec![0; buf_size];
            let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
            let status =
                tibco_ems_sys::tibemsDestination_GetName(jms_destination, buf_ref, buf_size);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetName: {:?}", status),
                _ => error!("tibemsDestination_GetName: {:?}", status),
            }
            let destination_name: String = CStr::from_ptr(buf_ref).to_str().unwrap().to_string();
            let jms_destination_obj: Option<Destination> = match destination_type {
                tibemsDestinationType::TIBEMS_QUEUE => Some(Destination::Queue(destination_name)),
                tibemsDestinationType::TIBEMS_TOPIC => Some(Destination::Topic(destination_name)),
                _ => {
                    //ignore unknown type
                    None
                }
            };
            //add replyTo to message
            match &mut msg {
                Message::TextMessage(msg) => msg.destination = jms_destination_obj,
                Message::BytesMessage(msg) => msg.destination = jms_destination_obj,
                Message::MapMessage(msg) => msg.destination = jms_destination_obj,
                Message::ObjectMessage(msg) => msg.destination = jms_destination_obj,
            }
        }
        // look for replyTo header
        let mut reply_destination: usize = 0;
        let status = tibco_ems_sys::tibemsMsg_GetReplyTo(msg_pointer, &mut reply_destination);
        match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetReplyTo: {:?}", status),
            _ => error!("tibemsMsg_GetReplyTo: {:?}", status),
        }
        if reply_destination != 0 {
            //has a destination
            let mut destination_type = tibemsDestinationType::TIBEMS_UNKNOWN;
            let status =
                tibco_ems_sys::tibemsDestination_GetType(reply_destination, &mut destination_type);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetType: {:?}", status),
                _ => error!("tibemsDestination_GetType: {:?}", status),
            }
            let buf_size = 1024;
            let buf_vec: Vec<i8> = vec![0; buf_size];
            let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
            let status =
                tibco_ems_sys::tibemsDestination_GetName(reply_destination, buf_ref, buf_size);
            match status {
                tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetName: {:?}", status),
                _ => error!("tibemsDestination_GetName: {:?}", status),
            }
            let destination_name: String = CStr::from_ptr(buf_ref).to_str().unwrap().to_string();
            let reply_destination_obj: Option<Destination> = match destination_type {
                tibemsDestinationType::TIBEMS_QUEUE => Some(Destination::Queue(destination_name)),
                tibemsDestinationType::TIBEMS_TOPIC => Some(Destination::Topic(destination_name)),
                _ => {
                    //ignore unknown type
                    None
                }
            };
            //add replyTo to message
            match &mut msg {
                Message::TextMessage(msg) => msg.reply_to = reply_destination_obj,
                Message::BytesMessage(msg) => msg.reply_to = reply_destination_obj,
                Message::MapMessage(msg) => msg.reply_to = reply_destination_obj,
                Message::ObjectMessage(msg) => msg.reply_to = reply_destination_obj,
            }
        }
    }
    msg
}
