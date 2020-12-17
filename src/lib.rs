//! Tibco EMS binding.

use std::ffi::CString;
use std::ffi::CStr;
use std::collections::HashMap;
use std::io::Error;
use tibco_ems_sys::tibems_status;
use tibco_ems_sys::tibemsDestinationType;
use log::{trace, error};

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
  pointer: usize
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
}

/// Type of the destination
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub enum DestinationType{
  /// destination type queue
  Queue,
  /// destination type topic
  Topic
}

/// open a connection to the Tibco EMS server
pub fn connect(url: String, user: String, password: String) -> Result<Connection, Error> {
  let conn: Connection;
  let mut connection_pointer: usize = 0;
  unsafe{
    let factory = tibco_ems_sys::tibemsConnectionFactory_Create();
    let status = tibco_ems_sys::tibemsConnectionFactory_SetServerURL(factory, CString::new(url).unwrap().as_ptr());
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsConnectionFactory_SetServerURL: {:?}",status),
      _ => error!("tibemsConnectionFactory_SetServerURL: {:?}",status),
    }
    let status = tibco_ems_sys::tibemsConnectionFactory_CreateConnection(factory,&mut connection_pointer,CString::new(user).unwrap().as_ptr(),CString::new(password).unwrap().as_ptr());
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
      session = Session{pointer: session_pointer};
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
      session = Session{pointer: session_pointer};
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
    let mut destination_pointer:usize = 0;
    unsafe{
      //create destination
      match destination.destination_type {
        DestinationType::Queue => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        },
        DestinationType::Topic => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        }
      }
      //open consumer
      let mut consumer_pointer:usize = 0;
      let selector_str;
      match selector {
        Some(val) => selector_str=CString::new(val).unwrap().as_ptr(),
        _ => selector_str = std::ptr::null(),
      }
      let status = tibco_ems_sys::tibemsSession_CreateConsumer(self.pointer, &mut consumer_pointer,destination_pointer, selector_str, tibco_ems_sys::tibems_bool::TIBEMS_TRUE);
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
      let status = tibco_ems_sys::tibemsSession_Close(self.pointer);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_Close: {:?}",status),
        _ => error!("tibemsSession_Close: {:?}",status),
      }
    }
  }

  /// sending a message to a destination (only queues are supported)
  pub fn send_message(&self, destination: Destination, message: Message) -> Result<(),Error>{
    let mut dest:usize = 0;
    unsafe{
      match destination.destination_type {
        DestinationType::Queue => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        },
        DestinationType::Topic => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        }
      }
      let mut producer: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(self.pointer,&mut producer,dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => error!("tibemsSession_CreateProducer: {:?}",status),
      }
      let mut msg: usize = 0;
      match message.message_type {
        MessageType::TextMessage =>{
          let status = tibco_ems_sys::tibemsTextMsg_Create(&mut msg);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_Create: {:?}",status),
            _ => error!("tibemsTextMsg_Create: {:?}",status),
          }
          let status = tibco_ems_sys::tibemsTextMsg_SetText(msg,CString::new(message.body_text.clone().unwrap()).unwrap().as_ptr());
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
        },
      }
      //set header
      match message.header.clone() {
        Some(headers)=>{
          for (key, val) in &headers {
            let status = tibco_ems_sys::tibemsMsg_SetStringProperty(msg, 
              CString::new(key.to_string()).unwrap().as_ptr(), 
              CString::new(val.to_string()).unwrap().as_ptr());
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetStringProperty: {:?}",status),
                _ => error!("tibemsMsg_SetStringProperty: {:?}",status),
              }
          }
        },
        None => {},
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
    }
    Ok(())
  }

  /// request/reply
  pub fn request_reply(&self, destination: Destination, message: Message, timeout: i64) -> Result<Option<Message>,Error>{
    //create temporary destination
    let mut reply_dest: usize = 0;
    let mut dest:usize = 0;
    unsafe {
      match destination.destination_type {
        DestinationType::Queue =>{
          let status = tibco_ems_sys::tibemsSession_CreateTemporaryQueue(self.pointer, &mut reply_dest);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateTemporaryQueue: {:?}",status),
            _ => error!("tibemsSession_CreateTemporaryQueue: {:?}",status),
          }
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
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
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsDestination_Create: {:?}",status),
            _ => error!("tibemsDestination_Create: {:?}",status),
          }
        }
      }
      let mut producer: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(self.pointer,&mut producer,dest);
      match status {
        tibems_status::TIBEMS_OK => trace!("tibemsSession_CreateProducer: {:?}",status),
        _ => error!("tibemsSession_CreateProducer: {:?}",status),
      }
      let mut msg: usize = 0;
      match message.message_type {
        MessageType::TextMessage =>{
          let status = tibco_ems_sys::tibemsTextMsg_Create(&mut msg);
          match status {
            tibems_status::TIBEMS_OK => trace!("tibemsTextMsg_Create: {:?}",status),
            _ => error!("tibemsTextMsg_Create: {:?}",status),
          }
          let status = tibco_ems_sys::tibemsTextMsg_SetText(msg,CString::new(message.body_text.clone().unwrap()).unwrap().as_ptr());
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
        },
      }
      //set header
      match message.header.clone() {
        Some(headers)=>{
          for (key, val) in &headers {
            let status = tibco_ems_sys::tibemsMsg_SetStringProperty(msg, 
              CString::new(key.to_string()).unwrap().as_ptr(), 
              CString::new(val.to_string()).unwrap().as_ptr());
              match status {
                tibems_status::TIBEMS_OK => trace!("tibemsMsg_SetStringProperty: {:?}",status),
                _ => error!("tibemsMsg_SetStringProperty: {:?}",status),
              }
          }
        },
        None => {},
      }
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
pub struct TextMessage{
  /// message body
  pub body: String,
  /// message header
  pub header: Option<HashMap<String,String>>,
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
pub struct BytesMessage{
  /// message body
  pub body: Vec<u8>,
  /// message header
  pub header: Option<HashMap<String,String>>,
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

/// represents a generic Message which can be transformed into a TextMessage or BytesMessage through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Message{
  /// type of the message, currenlty on TextMessage is supported
  pub message_type: MessageType,
  /// reply to header
  pub reply_to: Option<Destination>,
  /// message body if type is text
  body_text: Option<String>,
  /// message body if type is binary
  body_binary: Option<Vec<u8>>,
  // message header
  header: Option<HashMap<String,String>>,
  message_pointer: Option<usize>,
}

impl From<TextMessage> for Message {
  fn from(msg: TextMessage) -> Self {
    Message{
      message_type: MessageType::TextMessage,
      body_text: Some(msg.body.clone()),
      body_binary: None,
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

fn build_message_from_pointer(msg_pointer: usize) -> Message {
  let mut msg = Message{
    message_type: MessageType::TextMessage,
    body_text: None,
    body_binary: None,
    header: None,
    message_pointer: None,
    reply_to: None,
  };
  unsafe{
    let mut msg_type: tibco_ems_sys::tibemsMsgType = tibco_ems_sys::tibemsMsgType::TIBEMS_TEXT_MESSAGE;
    let status = tibco_ems_sys::tibemsMsg_GetBodyType(msg_pointer, &mut msg_type);
    match status {
      tibems_status::TIBEMS_OK => trace!("tibemsMsg_GetBodyType: {:?}",status),
      _ => error!("tibemsMsg_GetBodyType: {:?}",status),
    }
    match msg_type {
      tibco_ems_sys::tibemsMsgType::TIBEMS_TEXT_MESSAGE => {
        let mut header: HashMap<String,String> = HashMap::new();
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
        header.insert("MessageID".to_string(),message_id.to_string());
        msg = Message{
          message_type: MessageType::TextMessage,
          body_text: Some(content.to_string()),
          body_binary: None,
          header: Some(header),
          message_pointer: Some(msg_pointer),
          reply_to: None,
        };
      },
      _ => {
        //unknown
        println!("BodyType: {:?}",msg_type);
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
        tibco_ems_sys::tibems_status::TIBEMS_OK =>{
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
          header.insert(header_name.to_string(),header_value.to_string());
          msg.header=Some(header);
        }
        tibco_ems_sys::tibems_status::TIBEMS_NOT_FOUND =>{
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