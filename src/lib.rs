//! Tibco EMS binding.

use std::ffi::CString;
use std::ffi::CStr;
use std::ffi::c_void;
use std::collections::HashMap;
use std::io::Error;
use tibco_ems_sys::tibems_status;
use tibco_ems_sys::tibemsDestinationType;
use tibco_ems_sys::tibems_bool;
use tibco_ems_sys::tibemsMsgType;
use log::{trace, error};
use std::convert::TryInto;
use serde::{Serialize, Deserialize};

pub mod admin;

/// holds the native Connection pointer
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Connection{
  pointer: usize
}

/// holds the native Session pointer
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Session{
  pointer: usize,
  producer_pointer: usize,
}

/// holds the native Consumer pointer
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct Consumer{
  pointer: usize
}

/// Destination, can either be Queue or Topic
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Destination{
  /// type of the destination, either queue or topic
  pub destination_type: DestinationType,
  /// name of the destination
  pub destination_name: String,
}

/// Type of the message
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub enum MessageType{
  /// message body of type text
  TextMessage,
  /// message body of type binary
  BytesMessage,
  /// message body of type map
  MapMessage,
}

/// Type of the destination
#[allow(dead_code)]
#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub enum DestinationType{
  /// destination type queue
  Queue = 1,
  /// destination type topic
  Topic = 2,
}

/// open a connection to the Tibco EMS server
pub fn connect(url: &str, user: &str, password: &str) -> Result<Connection, Error> {
  let conn: Connection;
  let mut connection_pointer: usize = 0;
  unsafe{
    let factory = tibco_ems_sys::tibemsConnectionFactory_Create();
    let c_url = CString::new(url).unwrap();
    let status = tibco_ems_sys::tibemsConnectionFactory_SetServerURL(factory, c_url.as_ptr());
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnectionFactory_SetServerURL: {:?}",status),
      _ => error!("tibemsConnectionFactory_SetServerURL: {:?}",status),
    }
    let c_user = CString::new(user).unwrap();
    let c_password = CString::new(password).unwrap();
    let status = tibco_ems_sys::tibemsConnectionFactory_CreateConnection(factory,&mut connection_pointer,c_user.as_ptr(),c_password.as_ptr());
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnectionFactory_CreateConnection: {:?}",status),
      _ => error!("tibemsConnectionFactory_CreateConnection: {:?}",status),
    }
    conn = Connection{pointer: connection_pointer};
    let status = tibco_ems_sys::tibemsConnection_Start(connection_pointer);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnection_Start: {:?}",status),
      _ => error!("tibemsConnection_Start: {:?}",status),
    }
  }
  Ok(conn)
}

//
// connection
//

impl Connection {
  /// open a session
  pub fn session(&self)-> Result<Session,Error> {
    let session: Session;
    unsafe{
      let mut session_pointer:usize = 0;
      let status = tibco_ems_sys::tibemsConnection_CreateSession(self.pointer, &mut session_pointer, tibco_ems_sys::tibems_bool::TIBEMS_FALSE, tibco_ems_sys::tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsConnection_CreateSession: {:?}",status),
        _ => error!("tibemsConnection_CreateSession: {:?}",status),
      }
      let mut producer: usize = 0;
      let dest: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(session_pointer,&mut producer,dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => error!("tibemsSession_CreateProducer: {:?}",status),
      }
      session = Session{pointer: session_pointer, producer_pointer: producer};
    }
    Ok(session)
  }
  /// open a session with transaction support
  pub fn transacted_session(&self)-> Result<Session,Error> {
    let session: Session;
    unsafe{
      let mut session_pointer:usize = 0;
      let status = tibco_ems_sys::tibemsConnection_CreateSession(self.pointer, &mut session_pointer, tibco_ems_sys::tibems_bool::TIBEMS_FALSE, tibco_ems_sys::tibemsAcknowledgeMode::TIBEMS_EXPLICIT_CLIENT_ACKNOWLEDGE);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsConnection_CreateSession: {:?}",status),
        _ => error!("tibemsConnection_CreateSession: {:?}",status),
      }
      let mut producer: usize = 0;
      let dest: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(session_pointer,&mut producer,dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => error!("tibemsSession_CreateProducer: {:?}",status),
      }
      session = Session{pointer: session_pointer, producer_pointer: producer};
    }
    Ok(session)
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
  pub fn receive_message(&self, wait_time_ms: Option<i64>) -> Result<Option<Message>,Error> {
    unsafe{
      let mut msg_pointer:usize = 0;
      match wait_time_ms {
        Some(time_ms) => {
          let status = tibco_ems_sys::tibemsMsgConsumer_ReceiveTimeout(self.pointer, &mut msg_pointer, time_ms);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_ReceiveTimeout: {:?}",status),
            tibems_status::TIBEMS_TIMEOUT =>{
              return Ok(None);
            },
            _ => error!("tibemsMsgConsumer_ReceiveTimeout: {:?}",status),
          }
        },
        None => {
          let status = tibco_ems_sys::tibemsMsgConsumer_Receive(self.pointer, &mut msg_pointer);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsgConsumer_Receive: {:?}",status),
            _ => error!("tibemsMsgConsumer_Receive: {:?}",status),
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
  /// open a message consumer
  pub fn queue_consumer(&self, destination: Destination, selector: Option<String>)-> Result<Consumer,Error> {
    let consumer: Consumer;
    let mut destination_pointer: usize = 0;
    unsafe{
      //create destination
      match destination.destination_type {
        DestinationType::Queue => {
          let c_destination = CString::new(destination.destination_name).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_QUEUE, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        },
        DestinationType::Topic => {
          let c_destination = CString::new(destination.destination_name).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
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
        _ => error!("tibemsSession_CreateConsumer: {:?}",status),
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
  pub fn send_message(&self, destination: Destination, message: Message) -> Result<(),Error>{
    let mut dest: usize = 0;
    let mut local_producer: usize = 0;
    unsafe{
      match destination.destination_type {
        DestinationType::Queue => {
          let c_destination = CString::new(destination.destination_name).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        },
        DestinationType::Topic => {
          let c_destination = CString::new(destination.destination_name).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_TOPIC, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        }
      }
      if self.producer_pointer == 0 {
        let status = tibco_ems_sys::tibemsSession_CreateProducer(self.pointer,&mut local_producer,dest);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
          _ => error!("tibemsSession_CreateProducer: {:?}",status),
        }
      }
      let msg = build_message_pointer_from_message(&message);
      let status = tibco_ems_sys::tibemsMsgProducer_SendToDestination(
          self.producer_pointer, dest, msg);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsMsgProducer_Send: {:?}",status),
        _ => error!("tibemsMsgProducer_Send: {:?}",status),
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
  pub fn request_reply(&self, destination: Destination, message: Message, timeout: i64) -> Result<Option<Message>,Error>{
    //create temporary destination
    let mut reply_dest: usize = 0;
    let mut dest: usize = 0;
    unsafe {
      match destination.destination_type {
        DestinationType::Queue =>{
          let status = tibco_ems_sys::tibemsSession_CreateTemporaryQueue(self.pointer, &mut reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateTemporaryQueue: {:?}",status),
            _ => error!("tibemsSession_CreateTemporaryQueue: {:?}",status),
          }
          let c_destination = CString::new(destination.destination_name).unwrap();
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, c_destination.as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        },
        DestinationType::Topic =>{
          let status = tibco_ems_sys::tibemsSession_CreateTemporaryTopic(self.pointer, &mut reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateTemporaryTopic: {:?}",status),
            _ => error!("tibemsSession_CreateTemporaryTopic: {:?}",status),
          }
          let c_destination = CString::new(destination.destination_name).unwrap();
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

/// represents a Text Message which can be transformed into Message through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct TextMessage {
  /// message body
  pub body: String,
  /// message header
  pub header: Option<HashMap<String,TypedValue>>,
}

impl From<Message> for TextMessage {
  fn from(msg: Message) -> Self {
    TextMessage{
      body: msg.body_text.clone().unwrap(),
      header: msg.header.clone(),
    }
  }
}

impl From<&Message> for TextMessage {
  fn from(msg: &Message) -> Self {
    TextMessage{
      body: msg.body_text.clone().unwrap(),
      header: msg.header.clone(),
    }
  }
}

/// represents a Bytes Message which can be transformed into Message through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct BytesMessage {
  /// message body
  pub body: Vec<u8>,
  /// message header
  pub header: Option<HashMap<String,TypedValue>>,
}

impl From<Message> for BytesMessage {
  fn from(msg: Message) -> Self {
    BytesMessage{
      body: msg.body_binary.clone().unwrap(),
      header: msg.header.clone(),
    }
  }
}

impl From<&Message> for BytesMessage {
  fn from(msg: &Message) -> Self {
    BytesMessage{
      body: msg.body_binary.clone().unwrap(),
      header: msg.header.clone(),
    }
  }
}

/// represents a Map Message which can be transformed into Message through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone,Default,Serialize, Deserialize, PartialEq)]
pub struct MapMessage {
  /// message body map properties
  pub body: HashMap<String,TypedValue>,
  /// message header
  pub header: Option<HashMap<String,TypedValue>>,
}

impl From<Message> for MapMessage {
  fn from(msg: Message) -> Self {
    //use the borrow implementation
    (&msg).into()
  }
}

impl From<&Message> for MapMessage {
  fn from(msg: &Message) -> Self {
    let mut out_msg: MapMessage = Default::default();
    out_msg.header= msg.header.clone();
    let map = msg.body_map.clone().unwrap();
    for (key,val) in map {
      match val.value_type {
        PropertyType::String => {
          out_msg.body.insert(key, TypedValue{
            value_type: PropertyType::String,
            value: val.value,
          });
        },
        PropertyType::Map => {
          out_msg.body.insert(key, TypedValue{
            value_type: PropertyType::Map,
            value: val.value,
          });
        },
        _ => {
          panic!("missing body type implementation {:?}",val.value_type);
        }
      }
    }
    out_msg
  }
}

/// represents a generic Message which can be transformed into a TextMessage or BytesMessage through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Message {
  /// type of the message, currenlty on TextMessage is supported
  pub message_type: MessageType,
  /// reply to header
  pub reply_to: Option<Destination>,
  /// message body if type is text
  body_text: Option<String>,
  /// message body if type is binary
  body_binary: Option<Vec<u8>>,
  /// message body if type is map
  body_map: Option<HashMap<String, TypedValue>>,
  // message header
  header: Option<HashMap<String,TypedValue>>,
  message_pointer: Option<usize>,
}

/// represents a typed value, which is used for message header and message properties
#[allow(dead_code)]
#[derive(Debug,Clone,Serialize, Deserialize, PartialEq)]
pub struct TypedValue{
  /// type of the property
  pub value_type: PropertyType,
  /// binary representation of the value
  pub value: Vec<u8>,
}

impl TypedValue {
  /// constructs a TypedValue from a boolean
  pub fn bool_value(val: bool) -> TypedValue{
    match val {
      true => TypedValue{
        value_type: PropertyType::Boolean,
        value: vec![1],
       },
      false => TypedValue{
        value_type: PropertyType::Boolean,
        value: vec![0],
      },
    }
  }
  /// constructs a TypedValue from a i32
  pub fn int_value(val: i32) -> TypedValue{
    TypedValue{
      value_type: PropertyType::Integer,
      value: val.to_ne_bytes().to_vec(),
    }
  }
  /// constructs a TypedValue from a i64
  pub fn long_value(val: i64) -> TypedValue{
    TypedValue{
      value_type: PropertyType::Long,
      value: val.to_ne_bytes().to_vec(),
    }
  }
  /// constructs a TypedValue from a String
  pub fn string_value(val: String) -> TypedValue{
    TypedValue{
      value_type: PropertyType::String,
      value: val.as_bytes().to_vec(),
    }
  }
  /// constructs a TypedValue from a &[u8]
  pub fn binary_value(val: &[u8]) -> TypedValue{
    TypedValue{
      value_type: PropertyType::Binary,
      value: val.to_vec(),
    }
  }
  /// constructs a TypedValue from a f32
  pub fn float_value(val: f32) -> TypedValue{
    TypedValue{
      value_type: PropertyType::Float,
      value: val.to_ne_bytes().to_vec(),
    }
  }
  /// constructs a TypedValue from a f64
  pub fn double_value(val: f64) -> TypedValue{
    TypedValue{
      value_type: PropertyType::Double,
      value: val.to_ne_bytes().to_vec(),
    }
  }
  /// constructs a TypedValue from a MapMessage
  pub fn map_value(val: MapMessage) -> TypedValue{
    TypedValue{
      value_type: PropertyType::Map,
      value: bincode::serialize(&val).unwrap(),
    }
  }
}

/// Trait to retrieve a i32 value
pub trait GetIntValue {
  /// retrieve typed value
  fn int_value(&self) -> Result<i32,Error>;
}

/// Trait to retrieve a i64 value
pub trait GetLongValue {
  /// retrieve typed value
  fn long_value(&self) -> Result<i64,Error>;
}

/// Trait to retrieve a bool value
pub trait GetBoolValue {
  /// retrieve typed value
  fn bool_value(&self) -> Result<bool,Error>;
}

/// Trait to retrieve a String value
pub trait GetStringValue {
  /// retrieve typed value
  fn string_value(&self) -> Result<String,Error>;
}

/// Trait to retrieve a f32 value
pub trait GetFloatValue {
  /// retrieve typed value
  fn float_value(&self) -> Result<f32,Error>;
}

/// Trait to retrieve a f64 value
pub trait GetDoubleValue {
  /// retrieve typed value
  fn double_value(&self) -> Result<f64,Error>;
}

/// Trait to retrieve a MapMessage value
pub trait GetMapValue {
  /// retrieve typed value
  fn map_value(&self) -> Result<MapMessage,Error>;
}

impl GetIntValue for TypedValue {
  fn int_value(&self) -> Result<i32,Error>{
    match self.value_type {
      PropertyType::Integer => {
        let (int_bytes, _) = self.value.split_at(std::mem::size_of::<i32>());
        let value = i32::from_ne_bytes(int_bytes.try_into().unwrap());
        Ok(value)
      },
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not an int value")),
    }
  }
}

impl GetLongValue for TypedValue {
  fn long_value(&self) -> Result<i64,Error>{
    match self.value_type {
      PropertyType::Long => {
        let (long_bytes, _) = self.value.split_at(std::mem::size_of::<i64>());
        let value = i64::from_ne_bytes(long_bytes.try_into().unwrap());
        Ok(value)
      },
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not an long value")),
    }
  }
}

impl GetBoolValue for TypedValue {
  fn bool_value(&self) -> Result<bool,Error>{
    match self.value_type {
      PropertyType::Boolean => {
        if self.value[0] == 0 {
          return Ok(false)
        } else {
          return Ok(true)
        }
      },
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not a bool value")),
    }
  }
}

impl GetStringValue for TypedValue {
  fn string_value(&self) -> Result<String,Error>{
    match self.value_type {
      PropertyType::String => Ok(String::from_utf8(self.value.clone()).unwrap()),
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not a string value")),
    }
  }
}

impl GetFloatValue for TypedValue {
  fn float_value(&self) -> Result<f32,Error>{
    match self.value_type {
      PropertyType::Float => {
        let (float_bytes, _) = self.value.split_at(std::mem::size_of::<f32>());
        let value = f32::from_ne_bytes(float_bytes.try_into().unwrap());
        Ok(value)
      },
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not a float value")),
    }
  }
}

impl GetDoubleValue for TypedValue {
  fn double_value(&self) -> Result<f64,Error>{
    match self.value_type {
      PropertyType::Double => {
        let (double_bytes, _) = self.value.split_at(std::mem::size_of::<f64>());
        let value = f64::from_ne_bytes(double_bytes.try_into().unwrap());
        Ok(value)
      },
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not a double value")),
    }
  }
}

impl GetMapValue for TypedValue {
  fn map_value(&self) -> Result<MapMessage,Error>{
    match self.value_type {
      PropertyType::Map => {
        let value = bincode::deserialize(&self.value).unwrap();
        Ok(value)
      },
      _ => Err(Error::new(std::io::ErrorKind::InvalidData, "not a map value")),
    }
  }
}

/// Type of a property value
#[allow(dead_code)]
#[derive(Debug,Clone,Serialize, Deserialize, PartialEq)]
pub enum PropertyType{
  /// represents a String Value
  String,
  /// represents a integer Value
  Integer,
  /// represents a long Value
  Long,
  /// represents a float Value
  Float,
  /// represents a double Value
  Double,
  /// represents a binary Value
  Binary,
  /// represents a MapMessage Value
  Map,
  /// represents a boolean Value
  Boolean,
}

impl From<TextMessage> for Message {
  fn from(msg: TextMessage) -> Self {
    Message{
      message_type: MessageType::TextMessage,
      body_text: Some(msg.body.clone()),
      body_binary: None,
      body_map: None,
      header: msg.header.clone(),
      message_pointer: None,
      reply_to: None,
    }
  }
}

impl From<BytesMessage> for Message {
  fn from(msg: BytesMessage) -> Self {
    Message{
      message_type: MessageType::BytesMessage,
      body_text: None,
      body_binary: Some(msg.body.clone()),
      body_map: None,
      header: msg.header.clone(),
      message_pointer: None,
      reply_to: None,
    }
  }
}

impl From<MapMessage> for Message {
  fn from(msg: MapMessage) -> Self {
    Message{
      message_type: MessageType::MapMessage,
      body_text: None,
      body_binary: None,
      body_map: Some(msg.body),
      header: msg.header.clone(),
      message_pointer: None,
      reply_to: None,
    }
  }
}

impl Message{
  fn destroy(&self){
    match self.message_pointer{
      Some(pointer) => {
        unsafe{
          let status = tibco_ems_sys::tibemsMsg_Destroy(pointer);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_Destroy: {:?}",status),
            _ => error!("tibemsMsg_Destroy: {:?}",status),
          }
        }
      },
      None => {}
    }
  }
  /// confirms the message by invoking tibemsMsg_Acknowledge
  pub fn confirm(&self){
    match self.message_pointer{
      Some(pointer) => {
        unsafe{
          let status = tibco_ems_sys::tibemsMsg_Acknowledge(pointer);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_Acknowledge: {:?}",status),
            _ => error!("tibemsMsg_Acknowledge: {:?}",status),
          }
        }
      },
      None => {}
    }
  }
  /// rolls the message back by invoking tibemsMsg_Recover
  pub fn rollback(&self){
    match self.message_pointer{
      Some(pointer) => {
        unsafe{
          let status = tibco_ems_sys::tibemsMsg_Recover(pointer);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsMsg_Recover: {:?}",status),
            _ => error!("tibemsMsg_Recover: {:?}",status),
          }
        }
      },
      None => {}
    }
  }
}

impl Drop for Message {
  fn drop(&mut self) {
    self.destroy();
  }
}

fn build_message_pointer_from_message(message: &Message) -> usize {
  let mut msg: usize = 0;
  unsafe{
    match message.message_type {
      MessageType::TextMessage =>{
        let status = tibco_ems_sys::tibemsTextMsg_Create(&mut msg);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_Create: {:?}",status),
          _ => error!("tibemsTextMsg_Create: {:?}",status),
        }
        let c_text = CString::new(message.body_text.clone().unwrap()).unwrap();
        let status = tibco_ems_sys::tibemsTextMsg_SetText(msg,c_text.as_ptr());
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_SetText: {:?}",status),
          _ => error!("tibemsTextMsg_SetText: {:?}",status),
        }
      },
      MessageType::BytesMessage =>{
        let status = tibco_ems_sys::tibemsBytesMsg_Create(&mut msg);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsBytesMsg_Create: {:?}",status),
          _ => error!("tibemsBytesMsg_Create: {:?}",status),
        }
        let content = message.body_binary.clone().unwrap();
        let body_size = content.len();
        let body_ptr = content.as_ptr() as *const c_void;
        let status = tibco_ems_sys::tibemsBytesMsg_SetBytes(msg,body_ptr,body_size as u32);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsBytesMsg_SetBytes: {:?}",status),
          _ => error!("tibemsBytesMsg_SetBytes: {:?}",status),
        }
      },
      MessageType::MapMessage => {
        let status = tibco_ems_sys::tibemsMapMsg_Create(&mut msg);
        match status {
          tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_Create: {:?}",status),
          _ => error!("tibemsMapMsg_Create: {:?}",status),
        }
        for (key,val) in message.body_map.clone().unwrap() {
          let c_name = CString::new(key).unwrap();
          match val.value_type {
            PropertyType::Boolean => {
              let result = val.bool_value().unwrap();
              let val;
              if result {
                val = tibems_bool::TIBEMS_TRUE;
              }else{
                val = tibems_bool::TIBEMS_FALSE;
              }
              let status = tibco_ems_sys::tibemsMapMsg_SetBoolean(msg, c_name.as_ptr(), val);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetBoolean: {:?}",status),
                _ => error!("tibemsMapMsg_SetBoolean: {:?}",status),
              }
            },
            PropertyType::String => {
              let c_value = CString::new(val.string_value().unwrap()).unwrap();
              let status = tibco_ems_sys::tibemsMapMsg_SetString(msg, c_name.as_ptr(), c_value.as_ptr());
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetString: {:?}",status),
                _ => error!("tibemsMapMsg_SetString: {:?}",status),
              }
            },
            PropertyType::Integer => {
              let value = val.int_value().unwrap();
              let status = tibco_ems_sys::tibemsMapMsg_SetInt(msg, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetInt: {:?}",status),
                _ => error!("tibemsMapMsg_SetInt: {:?}",status),
              }
            },
            PropertyType::Long => {
              let value = val.long_value().unwrap();
              let status = tibco_ems_sys::tibemsMapMsg_SetLong(msg, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetLong: {:?}",status),
                _ => error!("tibemsMapMsg_SetLong: {:?}",status),
              }
            },
            PropertyType::Float => {
              let value = val.float_value().unwrap();
              let status = tibco_ems_sys::tibemsMapMsg_SetFloat(msg, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetFloat: {:?}",status),
                _ => error!("tibemsMapMsg_SetFloat: {:?}",status),
              }
            },

            PropertyType::Double => {
              let value = val.double_value().unwrap();
              let status = tibco_ems_sys::tibemsMapMsg_SetDouble(msg, c_name.as_ptr(), value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetDouble: {:?}",status),
                _ => error!("tibemsMapMsg_SetDouble: {:?}",status),
              }
            },
            PropertyType::Binary => {
              //TODO implement
              // let status = tibco_ems_sys::tibemsMapMsg_SetBytes(message: usize, name: *const c_char, bytes: *mut c_void, bytesSize: u64)Long(msg, c_name.as_ptr(), value);
              // match status {
                // tibems_status::TIBEMS_OK => trace!("tibemsMapMsg_SetLong: {:?}",status),
                // _ => error!("tibemsMapMsg_SetLong: {:?}",status),
              // }
            }
            _ => {
              panic!("missing map message type implementation for {:?}",val.value_type);
            },
          }
        }
      },
    }
    //set header
    match message.header.clone() {
      Some(headers)=>{
        for (key, val) in &headers {
          let c_name = CString::new(key.to_string()).unwrap();
          match val.value_type {
            PropertyType::String => {
              let c_val = CString::new(val.string_value().unwrap()).unwrap();
              let status = tibco_ems_sys::tibemsMsg_SetStringProperty(msg, c_name.as_ptr(), c_val.as_ptr());
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetStringProperty: {:?}",status),
                _ => error!("tibemsMsg_SetStringProperty: {:?}",status),
              }    
            },
            PropertyType::Boolean => {
              let mut bool_value = tibems_bool::TIBEMS_FALSE;
              if val.bool_value().unwrap() {
                bool_value = tibems_bool::TIBEMS_TRUE;
              }
              let status = tibco_ems_sys::tibemsMsg_SetBooleanProperty(msg, c_name.as_ptr(), bool_value);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetBooleanProperty: {:?}",status),
                _ => error!("tibemsMsg_SetBooleanProperty: {:?}",status),
              }
            },
            PropertyType::Integer => {
              let int_val = val.int_value().unwrap();
              let status = tibco_ems_sys::tibemsMsg_SetIntProperty(msg, c_name.as_ptr(), int_val);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetIntProperty: {:?}",status),
                _ => error!("tibemsMsg_SetIntProperty: {:?}",status),
              }    
            },
            PropertyType::Long => {
              let long_val = val.long_value().unwrap();
              let status = tibco_ems_sys::tibemsMsg_SetLongProperty(msg, c_name.as_ptr(), long_val);
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetLongProperty: {:?}",status),
                _ => error!("tibemsMsg_SetLongProperty: {:?}",status),
              }    
            },
            _ => {
              panic!("missing property type implementation for {:?}",val.value_type);
            }
          }
        }
      },
      None => {},
    }
  }
  msg
}

fn build_message_from_pointer(msg_pointer: usize) -> Message {
  let mut msg;
  unsafe{
    let mut msg_type: tibemsMsgType = tibemsMsgType::TIBEMS_TEXT_MESSAGE;
    let status = tibco_ems_sys::tibemsMsg_GetBodyType(msg_pointer, &mut msg_type);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetBodyType: {:?}",status),
      _ => error!("tibemsMsg_GetBodyType: {:?}",status),
    }
    match msg_type {
      tibemsMsgType::TIBEMS_TEXT_MESSAGE => {
        let mut header: HashMap<String,TypedValue> = HashMap::new();
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
        header.insert("MessageID".to_string(),TypedValue{
          value_type: PropertyType::String,
          value: message_id.to_string().as_bytes().to_vec()
        });
        msg = Message{
          message_type: MessageType::TextMessage,
          body_text: Some(content.to_string()),
          body_binary: None,
          body_map: None,
          header: Some(header),
          message_pointer: Some(msg_pointer),
          reply_to: None,
        };
      },
      tibemsMsgType::TIBEMS_MAP_MESSAGE => {
        let mut header: HashMap<String, TypedValue> = HashMap::new();
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
          header.insert("MessageID".to_string(),TypedValue{
            value_type: PropertyType::String,
            value: message_id.to_string().as_bytes().to_vec(),
          });  
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
              trace!("getting value for property: {}",header_name.to_string());
              let mut val_buf_vec:Vec<i8> = vec![0; 0];
              let mut val_buf_ref: *mut std::os::raw::c_char = val_buf_vec.as_mut_ptr();
              let status = tibco_ems_sys::tibemsMapMsg_GetString(msg_pointer, buf_ref, &mut val_buf_ref);
              match status {
                tibems_status::TIBEMS_OK =>{
                  trace!("tibemsMapMsg_GetString: {:?}",status);
                  if !val_buf_ref.is_null() {
                    let header_value = CStr::from_ptr(val_buf_ref).to_str().unwrap();
                    body_entries.insert(header_name.to_string(),TypedValue{
                      value_type: PropertyType::String,
                      value: header_value.as_bytes().to_vec()
                    });  
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
                  raw_message.message_pointer = None;
                  let header_value: MapMessage = raw_message.into();
                  body_entries.insert(header_name.to_string(),TypedValue{
                    value_type: PropertyType::Map,
                    value: bincode::serialize(&header_value).unwrap(),
                  });
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
        msg = Message{
          message_type: MessageType::MapMessage,
          body_text: None,
          body_binary: None,
          body_map: Some(body_entries),
          header: Some(header),
          message_pointer: Some(msg_pointer),
          reply_to: None,
        };
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
          let mut header = msg.header.clone().unwrap();
          header.insert(header_name.to_string(),TypedValue{
            value_type: PropertyType::String,
            value: header_value.to_string().as_bytes().to_vec(),
          });
          msg.header=Some(header);
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
      let destination_name = CStr::from_ptr(buf_ref).to_str().unwrap();
      match destination_type {
        tibemsDestinationType::TIBEMS_QUEUE =>{
          msg.reply_to = Some(Destination{
            destination_type: DestinationType::Queue,
            destination_name: destination_name.to_string(),
          });
        },
        tibemsDestinationType::TIBEMS_TOPIC =>{
          msg.reply_to = Some(Destination{
            destination_type: DestinationType::Topic,
            destination_name: destination_name.to_string(),
          });
        },
        _ =>{
          //ignore unknown type
        }
      }
    }
  }
  msg
}