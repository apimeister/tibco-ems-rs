//! Tibco EMS binding.

use std::ffi::CString;
use std::ffi::CStr;
use std::ffi::c_void;
use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
use tibco_ems_sys::tibems_status;
use tibco_ems_sys::tibemsDestinationType;
use tibco_ems_sys::tibems_bool;
use tibco_ems_sys::tibemsMsgType;
use log::{trace, error};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::ops::Deref;

pub mod admin;

#[cfg(feature = "streaming")]
pub mod stream;

/// holds the native Connection pointer
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Connection{
  pointer: Arc<usize>,
}

/// holds the native Session pointer
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Session{
  pointer: usize,
  producer_pointer: usize,
}

/// holds the native Consumer pointer
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Consumer{
  pointer: usize,
}

/// Destination, can either be Queue or Topic
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Destination{
  Queue(String),
  Topic(String),
}

/// represents a Text Message
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for TextMessage{
  fn default() -> Self {
    TextMessage{
      body: "".to_string(),
      header: None,
      destination: None,
      reply_to: None,
      pointer: None,
    }  
  }
}

/// represents a Binary Message
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for BytesMessage{
  fn default() -> Self {
    BytesMessage{
      body: vec![],
      header: None,
      destination: None,
      reply_to: None,
      pointer: None,
    }  
  }
}

/// represents a Map Message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Default for MapMessage{
  fn default() -> Self {
    MapMessage{
      body: HashMap::new(),
      header: None,
      destination: None,
      reply_to: None,
      pointer: None,
    }  
  }
}

/// Message enum wich represents the different message types
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Message {
  /// represents a Text Message
  TextMessage(TextMessage),
  /// represents a Binary Message
  BytesMessage(BytesMessage),
  /// represents a Map Message
  MapMessage(MapMessage),
}

/// open a connection to the Tibco EMS server
pub fn connect(url: &str, user: &str, password: &str) -> Result<Connection, Error> {
  let mut connection_pointer: usize = 0;
  unsafe{
    let factory = tibco_ems_sys::tibemsConnectionFactory_Create();
    let c_url = CString::new(url).unwrap();
    let status = tibco_ems_sys::tibemsConnectionFactory_SetServerURL(factory, c_url.as_ptr());
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnectionFactory_SetServerURL: {:?}",status),
      _ => {
        error!("tibemsConnectionFactory_SetServerURL: {:?}",status);
        return Err(Error::new(ErrorKind::InvalidData, "cannot set server url"));
      },
    }
    let c_user = CString::new(user).unwrap();
    let c_password = CString::new(password).unwrap();
    let status = tibco_ems_sys::tibemsConnectionFactory_CreateConnection(factory, &mut connection_pointer, c_user.as_ptr(),c_password.as_ptr());
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnectionFactory_CreateConnection: {:?}",status),
      _ => {
        error!("tibemsConnectionFactory_CreateConnection: {:?}",status);
        return Err(Error::new(ErrorKind::NotConnected, "cannot create connection"));
      },
    }
    let status = tibco_ems_sys::tibemsConnection_Start(connection_pointer);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnection_Start: {:?}",status),
      _ => {
        error!("tibemsConnection_Start: {:?}",status);
        return Err(Error::new(ErrorKind::NotConnected, "cannot start connection"));
      },
    }
  }
  let conn = Connection{pointer: Arc::from(connection_pointer)};
  Ok(conn)
}

//
// connection
//

impl<'stream> Connection {
  /// open a session
  pub fn session(&self) -> Result<Session, Error> {
    unsafe{
      let mut session_pointer:usize = 0;
      let connection_pointer = *self.pointer.deref();
      let status = tibco_ems_sys::tibemsConnection_CreateSession(connection_pointer, &mut session_pointer, tibco_ems_sys::tibems_bool::TIBEMS_FALSE, tibco_ems_sys::tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsConnection_CreateSession: {:?}",status),
        _ => {
          error!("tibemsConnection_CreateSession: {:?}",status);
          return Err(Error::new(ErrorKind::Other, "creating session failed"));
        },
      }
      let mut producer: usize = 0;
      let dest: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(session_pointer,&mut producer,dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => {
          error!("tibemsSession_CreateProducer: {:?}",status);
          return Err(Error::new(ErrorKind::Other, "creating producer failed"));
        },
      }
      let session = Session{pointer: session_pointer, producer_pointer: producer};
      Ok(session)
    }
  }

  /// open a session with transaction support
  pub fn transacted_session(&self)-> Result<Session, Error> {
    let session: Session;
    unsafe{
      let mut session_pointer:usize = 0;
      let connection_pointer = *self.pointer.deref();
      let status = tibco_ems_sys::tibemsConnection_CreateSession(connection_pointer, &mut session_pointer, tibco_ems_sys::tibems_bool::TIBEMS_FALSE, tibco_ems_sys::tibemsAcknowledgeMode::TIBEMS_EXPLICIT_CLIENT_ACKNOWLEDGE);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsConnection_CreateSession: {:?}",status),
        _ => {
          error!("tibemsConnection_CreateSession: {:?}",status);
          return Err(Error::new(ErrorKind::Other, "creating session failed"));
        },
      }
      let mut producer: usize = 0;
      let dest: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(session_pointer,&mut producer,dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => {
          error!("tibemsSession_CreateProducer: {:?}",status);
          return Err(Error::new(ErrorKind::Other, "creating producer failed"));
        },
      }
      session = Session{pointer: session_pointer, producer_pointer: producer};
    }
    Ok(session)
  }
  /// get active url from a ft connection
  /// this is only required for admin connections, 
  /// normal connections automatically choose the active server
  pub fn get_active_url(&self) -> Result<String, Error> {
    let connection_pointer = *self.pointer.deref();
    unsafe{
      let buf_vec:Vec<i8> = vec![0; 0];
      let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
      let status = tibco_ems_sys::tibemsConnection_GetActiveURL(connection_pointer, &buf_ref);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsConnection_GetActiveURL: {:?}",status),
        _ => {
          error!("tibemsConnection_GetActiveURL: {:?}",status);
          return Err(Error::new(ErrorKind::Other, "failed to retrieve active url"));
        },
      }
      let url = CStr::from_ptr(buf_ref).to_str().unwrap();
      return Ok(url.to_string());
    }
  }
  // open a consumer as stream of messages
  #[cfg(feature = "streaming")]
  pub fn open_stream<T: Into<Message>>(&'stream self,destination: &Destination, selector: Option<&str>) -> Result<stream::MessageStream<T>,Error> {
    let session = self.session().unwrap();
    let consumer = session.queue_consumer(destination, selector).unwrap();
    let stream = stream::MessageStream::<T>{
      connection: Rc::from(self.clone()),
      session: Rc::from(session),
      consumer: Rc::from(consumer),
      message: None,
    };
    Ok(stream)
  }
}

//
// consumer
//

impl Consumer {
  /// receive messages from a consumer
  /// 
  /// function return after wait time with a Message or None
  /// a wait time of None blocks until a message is available
  pub fn receive_message(&self, wait_time_ms: Option<i64>) -> Result<Option<Message>, Error> {
    unsafe{
      let mut msg_pointer:usize = 0;
      match wait_time_ms {
        Some(time_ms) => {
          let status = tibco_ems_sys::tibemsMsgConsumer_ReceiveTimeout(self.pointer, &mut msg_pointer, time_ms);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_ReceiveTimeout: {:?}",status),
            tibems_status::TIBEMS_TIMEOUT => {
              return Ok(None);
            },
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsMsgConsumer_ReceiveTimeout: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("receive message failed: {}",status_str)));
            },
          }
        },
        None => {
          let status = tibco_ems_sys::tibemsMsgConsumer_Receive(self.pointer, &mut msg_pointer);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_Receive: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsMsgConsumer_Receive: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("receive message failed: {}",status_str)));
            },
          }
        },
      }
      let msg = build_message_from_pointer(msg_pointer);
      return Ok(Some(msg));
    }
  }
}

// 
// session
//

impl Session {
  /// open a message consumer for a queue
  pub fn queue_consumer(&self, destination: &Destination, selector: Option<&str>) -> Result<Consumer, Error> {
    let consumer: Consumer;
    let mut destination_pointer: usize = 0;
    unsafe{
      //create destination
      match destination {
        Destination::Queue(name) => {
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_QUEUE, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsDestination_Create: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("create destination failed: {}",status_str)));
            },
          }
        },
        Destination::Topic(name) => {
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsDestination_Create: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("create destination failed: {}",status_str)));
            },
          }
        }
      }
      //open consumer
      let mut consumer_pointer:usize = 0;
      let c_selector:CString;
      match selector {
        Some(val) => c_selector=CString::new(val).unwrap(),
        _ => c_selector = CString::new("".to_string()).unwrap(),
      }
      let status = tibco_ems_sys::tibemsSession_CreateConsumer(self.pointer, &mut consumer_pointer,destination_pointer, c_selector.as_ptr(), tibco_ems_sys::tibems_bool::TIBEMS_TRUE);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateConsumer: {:?}",status),
        _ => {
          let status_str = format!("{:?}",status);
          error!("tibemsSession_CreateConsumer: {}",status_str);
          return Err(Error::new(ErrorKind::Other, format!("create consumer failed: {}",status_str)));
        },
      }
      consumer = Consumer{pointer: consumer_pointer};
    }
    Ok(consumer)
  }

  /// open a message consumer for a topic
  pub fn topic_consumer(&self, destination: &Destination, subscription_name: &str, selector: Option<&str>) -> Result<Consumer, Error> {
    let consumer: Consumer;
    let mut destination_pointer: usize = 0;
    unsafe{
      //create destination
      match destination {
        Destination::Topic(name) => {
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsDestination_Create: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("create destination failed: {}",status_str)));
            },
          }
        },
        Destination::Queue(_) => {
          return Err(Error::new(ErrorKind::Other, format!("destination is not of type topic")));
        }
      }
      //open consumer
      let mut consumer_pointer:usize = 0;
      let c_subscription_name = CString::new(subscription_name.clone()).unwrap();
      let c_selector:CString;
      match selector {
        Some(val) => c_selector=CString::new(val).unwrap(),
        _ => c_selector = CString::new("".to_string()).unwrap(),
      }
      let status = tibco_ems_sys::tibemsSession_CreateSharedConsumer(self.pointer, &mut consumer_pointer,destination_pointer, c_subscription_name.as_ptr(), c_selector.as_ptr());
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateSharedConsumer: {:?}",status),
        _ => {
          let status_str = format!("{:?}",status);
          error!("tibemsSession_CreateSharedConsumer: {}",status_str);
          return Err(Error::new(ErrorKind::Other, format!("create consumer failed: {}",status_str)));
        },
      }
      consumer = Consumer{pointer: consumer_pointer};
    }
    Ok(consumer)
  }

  /// open a durable message consumer for a topic
  pub fn topic_durable_consumer(&self, destination: &Destination, durable_name: &str, selector: Option<&str>) -> Result<Consumer, Error> {
    let consumer: Consumer;
    let mut destination_pointer: usize = 0;
    unsafe{
      //create destination
      match destination {
        Destination::Topic(name) => {
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsDestination_Create: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("create destination failed: {}",status_str)));
            },
          }
        },
        Destination::Queue(_) => {
          return Err(Error::new(ErrorKind::Other, format!("destination is not of type topic")));
        }
      }
      //open consumer
      let mut consumer_pointer:usize = 0;
      let c_durable_name = CString::new(durable_name.clone()).unwrap();
      let c_selector:CString;
      match selector {
        Some(val) => c_selector=CString::new(val).unwrap(),
        _ => c_selector = CString::new("".to_string()).unwrap(),
      }
      let status = tibco_ems_sys::tibemsSession_CreateSharedDurableConsumer(self.pointer, &mut consumer_pointer, destination_pointer, c_durable_name.as_ptr(), c_selector.as_ptr());
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateSharedDurableConsumer: {:?}",status),
        _ => {
          let status_str = format!("{:?}",status);
          error!("tibemsSession_CreateSharedDurableConsumer: {}",status_str);
          return Err(Error::new(ErrorKind::Other, format!("create consumer failed: {}",status_str)));
        },
      }
      consumer = Consumer{pointer: consumer_pointer};
    }
    Ok(consumer)
  }

  /// close a session
  fn close(&self){
    unsafe{
      //destroy producer
      if self.producer_pointer != 0 {
        let status = tibco_ems_sys::tibemsMsgProducer_Close(self.producer_pointer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Close: {:?}",status),
          _ => error!("tibemsMsgProducer_Close: {:?}",status),
        }
      }
      let status = tibco_ems_sys::tibemsSession_Close(self.pointer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_Close: {:?}",status),
        _ => error!("tibemsSession_Close: {:?}",status),
      }
    }
  }

  /// sending a message to a destination (only queues are supported)
  pub fn send_message<M: Into<Message>>(&self, destination: &Destination, message: M) -> Result<(), Error>{
    let message: Message = message.into();
    let mut dest: usize = 0;
    let mut local_producer: usize = 0;
    unsafe{
      match destination {
        Destination::Queue(name) => {
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsDestination_Create: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("create destination failed: {}",status_str)));
            },
          }
        },
        Destination::Topic(name) => {
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => {
              let status_str = format!("{:?}",status);
              error!("tibemsDestination_Create: {}",status_str);
              return Err(Error::new(ErrorKind::Other, format!("create destination failed: {}",status_str)));
            },
          }
        }
      }
      if self.producer_pointer == 0 {
        let status = tibco_ems_sys::tibemsSession_CreateProducer(self.pointer,&mut local_producer,dest);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
          _ =>{
            let status_str = format!("{:?}",status);
            error!("tibemsSession_CreateProducer: {}",status_str);
            return Err(Error::new(ErrorKind::Other, format!("create produce failed: {}",status_str)));
          },
        }
      }
      let msg = build_message_pointer_from_message(&message);
      let status = tibco_ems_sys::tibemsMsgProducer_SendToDestination(
          self.producer_pointer, dest, msg);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Send: {:?}",status),
        _ =>{
          let status_str = format!("{:?}",status);
          error!("tibemsMsgProducer_Send: {}",status_str);
          return Err(Error::new(ErrorKind::Other, format!("send message failed: {}",status_str)));
        },
      }
      //destroy producer if generated inline
      if self.producer_pointer == 0 {
        let status = tibco_ems_sys::tibemsMsgProducer_Close(local_producer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Close: {:?}",status),
          _ => error!("tibemsMsgProducer_Close: {:?}",status),
        }
      }
      //destroy message
      let status = tibco_ems_sys::tibemsMsg_Destroy(msg);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}",status),
        _ => error!("tibemsMsg_Destroy: {:?}",status),
      }
      //destroy destination
      let status = tibco_ems_sys::tibemsDestination_Destroy(dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsDestination_Destroy: {:?}",status),
        _ => error!("tibemsDestination_Destroy: {:?}",status),
      }
    }
    Ok(())
  }

  /// request/reply
  pub fn request_reply<M: Into<Message>>(&self, destination: &Destination, message: M, timeout: i64) -> Result<Option<Message>, Error>{
    let message: Message = message.into();
    //create temporary destination
    let mut reply_dest: usize = 0;
    let mut dest: usize = 0;
    unsafe {
      match &destination {
        Destination::Queue(name) =>{
          let status = tibco_ems_sys::tibemsSession_CreateTemporaryQueue(self.pointer, &mut reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateTemporaryQueue: {:?}",status),
            _ => error!("tibemsSession_CreateTemporaryQueue: {:?}",status),
          }
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        },
        Destination::Topic(name) =>{
          let status = tibco_ems_sys::tibemsSession_CreateTemporaryTopic(self.pointer, &mut reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateTemporaryTopic: {:?}",status),
            _ => error!("tibemsSession_CreateTemporaryTopic: {:?}",status),
          }
          let c_destination = CString::new(name.clone()).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        }
      }
      let mut producer: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(self.pointer,&mut producer, dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => error!("tibemsSession_CreateProducer: {:?}",status),
      }
      let msg = build_message_pointer_from_message(&message);
      //set reply to
      let status = tibco_ems_sys::tibemsMsg_SetReplyTo(msg, reply_dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetReplyTo: {:?}",status),
        _ => error!("tibemsMsg_SetReplyTo: {:?}",status),
      }
      let status = tibco_ems_sys::tibemsMsgProducer_Send(producer, msg);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Send: {:?}",status),
        _ => error!("tibemsMsgProducer_Send: {:?}",status),
      }
      //destroy message
      let status = tibco_ems_sys::tibemsMsg_Destroy(msg);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}",status),
        _ => error!("tibemsMsg_Destroy: {:?}",status),
      }
      //destroy producer
      let status = tibco_ems_sys::tibemsMsgProducer_Close(producer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Close: {:?}",status),
        _ => error!("tibemsMsgProducer_Close: {:?}",status),
      }
      //destroy destination
      let status = tibco_ems_sys::tibemsDestination_Destroy(dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsDestination_Destroy: {:?}",status),
        _ => error!("tibemsDestination_Destroy: {:?}",status),
      }
      //open consumer
      let mut consumer_pointer: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateConsumer(self.pointer, &mut consumer_pointer,reply_dest, std::ptr::null(), tibco_ems_sys::tibems_bool::TIBEMS_TRUE);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateConsumer: {:?}",status),
        _ => error!("tibemsSession_CreateConsumer: {:?}",status),
      }
      let mut reply_message: usize = 0;
      let status = tibco_ems_sys::tibemsMsgConsumer_ReceiveTimeout(consumer_pointer, &mut reply_message, timeout);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_ReceiveTimeout: {:?}",status),
        tibems_status::TIBEMS_TIMEOUT =>{
          return Ok(None);
        },
        _ => error!("tibemsMsgConsumer_ReceiveTimeout: {:?}",status),
      }
      let result = build_message_from_pointer(reply_message);
      //close consumer
      let status = tibco_ems_sys::tibemsMsgConsumer_Close(consumer_pointer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_Close: {:?}",status),
        _ => error!("tibemsMsgConsumer_Close: {:?}",status),
      }
      //destroy temporary destination
      match &destination {
        Destination::Queue{..} =>{
          //destroy reply_to_queue
          let status = tibco_ems_sys::tibemsSession_DeleteTemporaryQueue(self.pointer, reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_DeleteTemporaryQueue: {:?}",status),
            _ => error!("tibemsSession_DeleteTemporaryQueue: {:?}",status),
          }
        },
        Destination::Topic{..} =>{
          //destroy reply_to_queue
          let status = tibco_ems_sys::tibemsSession_DeleteTemporaryTopic(self.pointer, reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_DeleteTemporaryTopic: {:?}",status),
            _ => error!("tibemsSession_DeleteTemporaryTopic: {:?}",status),
          }
        }
      }
      return Ok(Some(result));
    }
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

/// represents a typed value, which is used for message header and message properties
#[allow(dead_code)]
#[derive(Debug,Clone,Serialize, Deserialize, PartialEq)]
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

impl Message{
  fn destroy(&self){
    let destroy_msg = | pointer: usize| unsafe {
      let status = tibco_ems_sys::tibemsMsg_Destroy(pointer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}",status),
        _ => error!("tibemsMsg_Destroy: {:?}",status),
      }
    };
    match self {
      Message::TextMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            destroy_msg(pointer);
          },
          None => {},
        }
      },
      Message::BytesMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            destroy_msg(pointer);
          },
          None => {},
        }
      },
      Message::MapMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            destroy_msg(pointer);
          },
          None => {},
        }
      },
    }
  }
  /// confirms the message by invoking tibemsMsg_Acknowledge
  pub fn confirm(&self){
    let ack_msg = |pointer:usize| unsafe {
      let status = tibco_ems_sys::tibemsMsg_Acknowledge(pointer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsg_Acknowledge: {:?}",status),
        _ => error!("tibemsMsg_Acknowledge: {:?}",status),
      }
    };
    match self {
      Message::TextMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            ack_msg(pointer);
          },
          None => {},
        }
      },
      Message::BytesMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            ack_msg(pointer);
          },
          None => {},
        }
      },
      Message::MapMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            ack_msg(pointer);
          },
          None => {},
        }
      },
    }
  }
  /// rolls the message back by invoking tibemsMsg_Recover
  pub fn rollback(&self){
    let recover = |pointer: usize| unsafe{
      let status = tibco_ems_sys::tibemsMsg_Recover(pointer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsg_Recover: {:?}",status),
        _ => error!("tibemsMsg_Recover: {:?}",status),
      }
    };
    match self {
      Message::TextMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            recover(pointer);
          },
          None => {},
        }
      },
      Message::BytesMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            recover(pointer);
          },
          None => {},
        }
      },
      Message::MapMessage(msg) => {
        match msg.pointer {
          Some(pointer) => {
            recover(pointer);
          },
          None => {},
        }
      },
    }
  }
}

impl Drop for Message {
  fn drop(&mut self) {
    self.destroy();
  }
}

fn build_message_pointer_from_message(message: &Message) -> usize {
  let mut msg_pointer: usize = 0;
  unsafe{
    match message {
      Message::TextMessage(msg) => {
        let status = tibco_ems_sys::tibemsTextMsg_Create(&mut msg_pointer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_Create: {:?}",status),
          _ => error!("tibemsTextMsg_Create: {:?}",status),
        }
        let c_text = CString::new(msg.body.clone()).unwrap();
        let status = tibco_ems_sys::tibemsTextMsg_SetText(msg_pointer,c_text.as_ptr());
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_SetText: {:?}",status),
          _ => error!("tibemsTextMsg_SetText: {:?}",status),
        }
      },
      Message::BytesMessage(msg) => {
        let status = tibco_ems_sys::tibemsBytesMsg_Create(&mut msg_pointer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsBytesMsg_Create: {:?}",status),
          _ => error!("tibemsBytesMsg_Create: {:?}",status),
        }
        let content = msg.body.clone();
        let body_size = content.len();
        let body_ptr = content.as_ptr() as *const c_void;
        let status = tibco_ems_sys::tibemsBytesMsg_SetBytes(msg_pointer,body_ptr,body_size as u32);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsBytesMsg_SetBytes: {:?}",status),
          _ => error!("tibemsBytesMsg_SetBytes: {:?}",status),
        }
      },
      Message::MapMessage(msg) => {
        let status = tibco_ems_sys::tibemsMapMsg_Create(&mut msg_pointer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_Create: {:?}",status),
          _ => error!("tibemsMapMsg_Create: {:?}",status),
        }
        for (key,val) in msg.body.clone() {
          let c_name = CString::new(key).unwrap();
          match val {
            TypedValue::Boolean(value) => {
              let status;
              if value {
                status = tibco_ems_sys::tibemsMapMsg_SetBoolean(msg_pointer, c_name.as_ptr(), tibems_bool::TIBEMS_TRUE);
              }else{
                status = tibco_ems_sys::tibemsMapMsg_SetBoolean(msg_pointer, c_name.as_ptr(), tibems_bool::TIBEMS_FALSE);
              }
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetBoolean: {:?}",status),
                _ => error!("tibemsMapMsg_SetBoolean: {:?}",status),
              }
            },
            TypedValue::String(value) => {
              let c_value = CString::new(value).unwrap();
              let status = tibco_ems_sys::tibemsMapMsg_SetString(msg_pointer, c_name.as_ptr(), c_value.as_ptr());
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetString: {:?}",status),
                _ => error!("tibemsMapMsg_SetString: {:?}",status),
              }
            },
            TypedValue::Integer(value) => {
              let status = tibco_ems_sys::tibemsMapMsg_SetInt(msg_pointer, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetInt: {:?}",status),
                _ => error!("tibemsMapMsg_SetInt: {:?}",status),
              }
            },
            TypedValue::Long(value) => {
              let status = tibco_ems_sys::tibemsMapMsg_SetLong(msg_pointer, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetLong: {:?}",status),
                _ => error!("tibemsMapMsg_SetLong: {:?}",status),
              }
            },
            TypedValue::Float(value) => {
              let status = tibco_ems_sys::tibemsMapMsg_SetFloat(msg_pointer, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetFloat: {:?}",status),
                _ => error!("tibemsMapMsg_SetFloat: {:?}",status),
              }
            },

            TypedValue::Double(value) => {
              let status = tibco_ems_sys::tibemsMapMsg_SetDouble(msg_pointer, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetDouble: {:?}",status),
                _ => error!("tibemsMapMsg_SetDouble: {:?}",status),
              }
            },
            TypedValue::Binary(_value) => {
              //TODO implement
              // let status = tibco_ems_sys::tibemsMapMsg_SetBytes(message: usize, name: *const c_char, bytes: *mut c_void, bytesSize: u64)Long(msg, c_name.as_ptr(), value);
              // match status {
                // tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetLong: {:?}",status),
                // _ => error!("tibemsMapMsg_SetLong: {:?}",status),
              // }
            }
            _ => {
              panic!("missing map message type implementation for {:?}",val);
            },
          }
        }
      },
    }
    //set header
    let header = match message {
      Message::TextMessage(msg) => msg.header.clone(),
      Message::BytesMessage(msg) => msg.header.clone(),
      Message::MapMessage(msg) => msg.header.clone(),
    };
    match header {
      Some(headers)=>{
        for (key, val) in &headers {
          let c_name = CString::new(key.to_string()).unwrap();
          match val {
            TypedValue::String(value) => {
              let c_val = CString::new(value.as_bytes()).unwrap();
              let status = tibco_ems_sys::tibemsMsg_SetStringProperty(msg_pointer, c_name.as_ptr(), c_val.as_ptr());
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetStringProperty: {:?}",status),
                _ => error!("tibemsMsg_SetStringProperty: {:?}",status),
              }    
            },
            TypedValue::Boolean(value) => {
              let status;
              if *value {
                status = tibco_ems_sys::tibemsMsg_SetBooleanProperty(msg_pointer, c_name.as_ptr(), tibems_bool::TIBEMS_TRUE);
              } else {
                status = tibco_ems_sys::tibemsMsg_SetBooleanProperty(msg_pointer, c_name.as_ptr(), tibems_bool::TIBEMS_FALSE);
              }
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetBooleanProperty: {:?}",status),
                _ => error!("tibemsMsg_SetBooleanProperty: {:?}",status),
              }
            },
            TypedValue::Integer(value) => {
              let status = tibco_ems_sys::tibemsMsg_SetIntProperty(msg_pointer, c_name.as_ptr(), *value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetIntProperty: {:?}",status),
                _ => error!("tibemsMsg_SetIntProperty: {:?}",status),
              }    
            },
            TypedValue::Long(value) => {
              let status = tibco_ems_sys::tibemsMsg_SetLongProperty(msg_pointer, c_name.as_ptr(), *value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetLongProperty: {:?}",status),
                _ => error!("tibemsMsg_SetLongProperty: {:?}",status),
              }    
            },
            _ => {
              panic!("missing property type implementation for {:?}",val);
            }
          }
        }
      },
      None => {},
    }
  }
  msg_pointer
}

fn build_message_from_pointer(msg_pointer: usize) -> Message {
  let mut msg: Message;
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  unsafe{
    let mut msg_type: tibemsMsgType = tibemsMsgType::TIBEMS_TEXT_MESSAGE;
    let status = tibco_ems_sys::tibemsMsg_GetBodyType(msg_pointer, &mut msg_type);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetBodyType: {:?}",status),
      _ => error!("tibemsMsg_GetBodyType: {:?}",status),
    }
    match msg_type {
      tibemsMsgType::TIBEMS_TEXT_MESSAGE => {
        let buf_vec:Vec<i8> = vec![0; 0];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsTextMsg_GetText(msg_pointer, & buf_ref);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_GetText: {:?}",status),
          _ => error!("tibemsTextMsg_GetText: {:?}",status),
        }
        let content = CStr::from_ptr(buf_ref).to_str().unwrap();
        let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}",status),
          _ => error!("tibemsMsg_GetMessageID: {:?}",status),
        }
        let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
        header.insert("MessageID".to_string(),TypedValue::String(message_id.to_string()));
        msg = Message::TextMessage(TextMessage{
          body: content.to_string(),
          header: None,
          pointer: Some(msg_pointer),
          destination: None,
          reply_to: None,
        });
      },
      tibemsMsgType::TIBEMS_MAP_MESSAGE => {
        let buf_vec:Vec<i8> = vec![0; 0];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}",status),
          _ => error!("tibemsMsg_GetMessageID: {:?}",status),
        }
        //admin messages do not have a message id
        if buf_vec.len() > 0 {
          let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
          &header.insert("MessageID".to_string(),TypedValue::String(message_id.to_string()));  
        }
        let mut names_pointer: usize = 0;
        trace!("tibemsMapMsg_GetMapNames");
        let status = tibco_ems_sys::tibemsMapMsg_GetMapNames(msg_pointer, &mut names_pointer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_GetMapNames: {:?}",status),
          _ => error!("tibemsMapMsg_GetMapNames: {:?}",status),
        }
        let mut body_entries: HashMap<String, TypedValue> = HashMap::new();
        loop {
          let buf_vec:Vec<i8> = vec![0; 0];
          let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
          let status = tibco_ems_sys::tibemsMsgEnum_GetNextName(names_pointer, &buf_ref);
          match status {
            tibems_status::TIBEMS_OK =>{
              let header_name = CStr::from_ptr(buf_ref).to_str().unwrap();
              trace!("getting value for property: {}",header_name);
              let mut val_buf_vec:Vec<i8> = vec![0; 0];
              let mut val_buf_ref: *mut std::os::raw::c_char = val_buf_vec.as_mut_ptr();
              let status = tibco_ems_sys::tibemsMapMsg_GetString(msg_pointer, buf_ref, &mut val_buf_ref);
              match status {
                tibems_status::TIBEMS_OK =>{
                  trace!("tibemsMapMsg_GetString: {:?}",status);
                  if !val_buf_ref.is_null() {
                    let header_value = CStr::from_ptr(val_buf_ref).to_str().unwrap();
                    body_entries.insert(header_name.to_string(),TypedValue::String(header_value.to_string()));  
                  }
                },
                tibems_status::TIBEMS_CONVERSION_FAILED => {
                  let mut msg2: usize = 0;
                  let status = tibco_ems_sys::tibemsMapMsg_GetMapMsg(msg_pointer, buf_ref, &mut msg2);
                  match status {
                    tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_GetMapMsg: {:?}",status),
                    _ => error!("tibemsMapMsg_GetMapMsg: {:?}",status),
                  }
                  let mut raw_message = build_message_from_pointer(msg2);
                  match &mut raw_message {
                    Message::TextMessage(_msg) => {},
                    Message::BytesMessage(_msg) => {},
                    Message::MapMessage(msg) => {
                      msg.pointer=None;
                      body_entries.insert(header_name.to_string(), TypedValue::Map(msg.clone()));
                    },
                  }
                },
                _ => error!("tibemsMapMsg_GetString: {:?}",status),
              }
              
            }
            tibems_status::TIBEMS_NOT_FOUND =>{
              break;
            }
            _ => {
              println!("tibemsMsgEnum_GetNextName: {:?}",status);
              break;
            }
          }
        }
        let status = tibco_ems_sys::tibemsMsgEnum_Destroy(names_pointer);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsgEnum_Destroy: {:?}",status),
          _ => error!("tibemsMsgEnum_Destroy: {:?}",status),
        }
        msg = Message::MapMessage(MapMessage{
          body: body_entries,
          header: None,
          pointer: Some(msg_pointer),
          destination: None,
          reply_to: None,
        });
      },
      tibemsMsgType::TIBEMS_BYTES_MESSAGE => {
        let buf_vec:Vec<i8> = vec![0; 0];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}",status),
          _ => error!("tibemsMsg_GetMessageID: {:?}",status),
        }
        let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
        header.insert("MessageID".to_string(),TypedValue::String(message_id.to_string()));
        msg = Message::BytesMessage(BytesMessage{
          body: vec![],
          header: None,
          pointer: Some(msg_pointer),
          destination: None,
          reply_to: None,
        });
      },
      tibemsMsgType::TIBEMS_OBJECT_MESSAGE => {
        let buf_vec:Vec<i8> = vec![0; 0];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetMessageID: {:?}",status),
          _ => error!("tibemsMsg_GetMessageID: {:?}",status),
        }
        let message_id = CStr::from_ptr(buf_ref).to_str().unwrap();
        header.insert("MessageID".to_string(),TypedValue::String(message_id.to_string()));
        msg = Message::BytesMessage(BytesMessage{
          body: vec![],
          header: None,
          pointer: Some(msg_pointer),
          destination: None,
          reply_to: None,
        });
      },
      _ => {
        //unknown
        panic!("BodyType {:?} not implemented",msg_type);
      }
    }
    // fetch header
    let mut header_enumeration: usize = 0;
    let status = tibco_ems_sys::tibemsMsg_GetPropertyNames(msg_pointer, &mut header_enumeration);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetPropertyNames: {:?}",status),
      _ => error!("tibemsMsg_GetPropertyNames: {:?}",status),
    }
    loop {
      let buf_vec:Vec<i8> = vec![0; 0];
      let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
      let status = tibco_ems_sys::tibemsMsgEnum_GetNextName(header_enumeration, &buf_ref);
      match status {
        tibems_status::TIBEMS_OK =>{
          let header_name = CStr::from_ptr(buf_ref).to_str().unwrap();
          let val_buf_vec:Vec<i8> = vec![0; 0];
          let val_buf_ref: *const std::os::raw::c_char = val_buf_vec.as_ptr();
          let status = tibco_ems_sys::tibemsMsg_GetStringProperty(msg_pointer, buf_ref, &val_buf_ref);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetStringProperty: {:?}",status),
            _ => error!("tibemsMsg_GetStringProperty: {:?}",status),
          }
          let header_value = CStr::from_ptr(val_buf_ref).to_str().unwrap();
          header.insert(header_name.to_string(),TypedValue::String(header_value.to_string()));
        }
        tibems_status::TIBEMS_NOT_FOUND =>{
          break;
        }
        _ => {
          println!("tibemsMsgEnum_GetNextName: {:?}",status);
          break;
        }
      }
    }
    let status = tibco_ems_sys::tibemsMsgEnum_Destroy(header_enumeration);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsgEnum_Destroy: {:?}",status),
      _ => error!("tibemsMsgEnum_Destroy: {:?}",status),
    }
    //add header to message
    match &mut msg {
      Message::TextMessage(msg) => msg.header=Some(header),
      Message::BytesMessage(msg) => msg.header=Some(header),
      Message::MapMessage(msg) => msg.header=Some(header),
    }
    // look for JMSDestination header
    let mut jms_destination: usize = 0;
    let status = tibco_ems_sys::tibemsMsg_GetDestination(msg_pointer, &mut jms_destination);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetDestination: {:?}",status),
      _ => error!("tibemsMsg_GetDestination: {:?}",status),
    }
    if jms_destination != 0 {
      //has a destination
      let mut destination_type = tibemsDestinationType::TIBEMS_UNKNOWN;
      let status = tibco_ems_sys::tibemsDestination_GetType(jms_destination, &mut destination_type);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetType: {:?}",status),
        _ => error!("tibemsDestination_GetType: {:?}",status),
      }
      let buf_size = 1024;
      let buf_vec:Vec<i8> = vec![0; buf_size];
      let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
      let status = tibco_ems_sys::tibemsDestination_GetName(jms_destination, buf_ref, buf_size);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetName: {:?}",status),
        _ => error!("tibemsDestination_GetName: {:?}",status),
      }
      let destination_name: String = CStr::from_ptr(buf_ref).to_str().unwrap().to_string();
      let jms_destination_obj: Option<Destination>;
      match destination_type {
        tibemsDestinationType::TIBEMS_QUEUE =>{
          jms_destination_obj = Some(Destination::Queue(destination_name));
        },
        tibemsDestinationType::TIBEMS_TOPIC =>{
          jms_destination_obj = Some(Destination::Topic(destination_name));
        },
        _ =>{
          //ignore unknown type
          jms_destination_obj = None;
        }
      }
      //add replyTo to message
      match &mut msg {
        Message::TextMessage(msg) => msg.destination=jms_destination_obj,
        Message::BytesMessage(msg) => msg.destination=jms_destination_obj,
        Message::MapMessage(msg) => msg.destination=jms_destination_obj,
      }
    }
    // look for replyTo header
    let mut reply_destination: usize = 0;
    let status = tibco_ems_sys::tibemsMsg_GetReplyTo(msg_pointer, &mut reply_destination);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetReplyTo: {:?}",status),
      _ => error!("tibemsMsg_GetReplyTo: {:?}",status),
    }
    if reply_destination != 0 {
      //has a destination
      let mut destination_type = tibemsDestinationType::TIBEMS_UNKNOWN;
      let status = tibco_ems_sys::tibemsDestination_GetType(reply_destination, &mut destination_type);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetType: {:?}",status),
        _ => error!("tibemsDestination_GetType: {:?}",status),
      }
      let buf_size = 1024;
      let buf_vec:Vec<i8> = vec![0; buf_size];
      let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
      let status = tibco_ems_sys::tibemsDestination_GetName(reply_destination, buf_ref, buf_size);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsDestination_GetName: {:?}",status),
        _ => error!("tibemsDestination_GetName: {:?}",status),
      }
      let destination_name: String = CStr::from_ptr(buf_ref).to_str().unwrap().to_string();
      let reply_destination_obj: Option<Destination>;
      match destination_type {
        tibemsDestinationType::TIBEMS_QUEUE =>{
          reply_destination_obj = Some(Destination::Queue(destination_name));
        },
        tibemsDestinationType::TIBEMS_TOPIC =>{
          reply_destination_obj = Some(Destination::Topic(destination_name));
        },
        _ =>{
          //ignore unknown type
          reply_destination_obj = None;
        }
      }
      //add replyTo to message
      match &mut msg {
        Message::TextMessage(msg) => msg.reply_to=reply_destination_obj,
        Message::BytesMessage(msg) => msg.reply_to=reply_destination_obj,
        Message::MapMessage(msg) => msg.reply_to=reply_destination_obj,
      }
    }      
  }
  msg
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_connection_failure() -> Result<(), String>{
    let result = connect("tcp://example.org:7222", "admin", "admin");
    match result{
      Ok(_val) => {
        return Err("no error was returned".to_string());
      },
      Err(_err) => {
        return Ok(());
      },
    }
  }
}