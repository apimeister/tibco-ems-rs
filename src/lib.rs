//! Tibco EMS binding.

use std::ffi::CString;
use std::ffi::CStr;
use std::collections::HashMap;
use std::io::Error;

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
    println!("tibemsConnectionFactory_SetServerURL: {:?}",status);
    let status = tibco_ems_sys::tibemsConnectionFactory_CreateConnection(factory,&mut connection_pointer,CString::new(user).unwrap().as_ptr(),CString::new(password).unwrap().as_ptr());
    println!("tibemsConnectionFactory_CreateConnection: {:?}",status);
    conn = Connection{pointer: connection_pointer};
    let status = tibco_ems_sys::tibemsConnection_Start(connection_pointer);
    println!("tibemsConnectionFactory_CreateConnection: {:?}",status);
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
      println!("tibemsConnection_CreateSession: {:?}",status);
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
    let mut msg:Message = Message{
      message_type: MessageType::TextMessage,
      body_text: None,
      body_binary: None,
      header: None,
      message_pointer: None,
      reply_to: None,
    };
    unsafe{
      let mut msg_pointer:usize = 0;
      match wait_time_ms {
        Some(time_ms) => {
          let status = tibco_ems_sys::tibemsMsgConsumer_ReceiveTimeout(self.pointer, &mut msg_pointer, time_ms);
          println!("tibemsMsgConsumer_Receive: {:?}",status);
          if status == tibco_ems_sys::tibems_status::TIBEMS_TIMEOUT {
            return Ok(None)
          }
        },
        None => {
          let status = tibco_ems_sys::tibemsMsgConsumer_Receive(self.pointer, &mut msg_pointer);
          println!("tibemsMsgConsumer_Receive: {:?}",status);    
        },
      }
      let mut msg_type: tibco_ems_sys::tibemsMsgType = tibco_ems_sys::tibemsMsgType::TIBEMS_TEXT_MESSAGE;
      let status = tibco_ems_sys::tibemsMsg_GetBodyType(msg_pointer, &mut msg_type);
      println!("tibemsMsg_GetBodyType: {:?}",status);
      match msg_type {
        tibco_ems_sys::tibemsMsgType::TIBEMS_TEXT_MESSAGE => {
          let mut header: HashMap<String,String> = HashMap::new();
          let buf_vec:Vec<i8> = vec![0; 0];
          let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
          let status = tibco_ems_sys::tibemsTextMsg_GetText(msg_pointer, & buf_ref);
          println!("tibemsTextMsg_GetText: {:?}",status);
          let content = CStr::from_ptr(buf_ref).to_str().unwrap();
          let status = tibco_ems_sys::tibemsMsg_GetMessageID(msg_pointer, &buf_ref);
          println!("tibemsMsg_GetMessageID: {:?}",status);
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
      println!("tibemsMsg_GetPropertyNames: {:?}",status);
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
            println!("tibemsMsg_GetStringProperty: {:?}",status);
            let header_value = CStr::from_ptr(val_buf_ref).to_str().unwrap();
            let mut header = msg.header.clone().unwrap();
            header.insert(header_name.to_string(),header_value.to_string());
            msg.header=Some(header);
          }
          _ => {
            println!("tibemsMsgEnum_GetNextName: {:?}",status);
            break;
          }
        }
      }
      let status = tibco_ems_sys::tibemsMsgEnum_Destroy(header_enumeration);
      println!("tibemsMsgEnum_Destroy: {:?}",status);
      // look for replyTo header
      let mut reply_destination: usize = 0;
      let status = tibco_ems_sys::tibemsMsg_GetReplyTo(msg_pointer, &mut reply_destination);
      println!("tibemsMsgEnum_Destroy: {:?}",status);
      if reply_destination != 0 {
        //has a destination
        let mut destination_type = tibco_ems_sys::tibemsDestinationType::TIBEMS_UNKNOWN;
        let status = tibco_ems_sys::tibemsDestination_GetType(reply_destination, &mut destination_type);
        println!("tibemsDestination_GetType: {:?}",status);
        let buf_size = 1024;
        let buf_vec:Vec<i8> = vec![0; buf_size];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = tibco_ems_sys::tibemsDestination_GetName(reply_destination, buf_ref, buf_size);
        println!("tibemsDestination_GetName: {:?}",status);
        let destination_name = CStr::from_ptr(buf_ref).to_str().unwrap();
        match destination_type {
          tibco_ems_sys::tibemsDestinationType::TIBEMS_QUEUE =>{
            msg.reply_to = Some(Destination{
              destination_type: DestinationType::Queue,
              destination_name: destination_name.to_string(),
            });
          },
          tibco_ems_sys::tibemsDestinationType::TIBEMS_TOPIC =>{
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
      println!("reply destination: {}",reply_destination);
    }
    Ok(Some(msg))
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
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibco_ems_sys::tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
          println!("tibemsDestination_Create: {:?}",status);
        },
        DestinationType::Topic => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut destination_pointer, tibco_ems_sys::tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
          println!("tibemsDestination_Create: {:?}",status);
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
      println!("tibemsSession_CreateConsumer: {:?}",status);
      consumer = Consumer{pointer: consumer_pointer};
    }
    Ok(consumer)
  }

  /// close a session
  fn close(&self){
    unsafe{
      let status = tibco_ems_sys::tibemsSession_Close(self.pointer);
      println!("tibemsSession_Close: {:?}",status);
    }
  }

  /// sending a message to a destination (only queues are supported)
  pub fn send_message(&self, destination: Destination, message: Message) -> Result<(),Error>{
    let mut dest:usize = 0;
    unsafe{
      match destination.destination_type {
        DestinationType::Queue => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibco_ems_sys::tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
          println!("tibemsDestination_Create: {:?}",status);
        },
        DestinationType::Topic => {
          let status = tibco_ems_sys::tibemsDestination_Create(&mut dest, tibco_ems_sys::tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
          println!("tibemsDestination_Create: {:?}",status);
        }
      }
      let mut producer: usize = 0;
      let status = tibco_ems_sys::tibemsSession_CreateProducer(self.pointer,&mut producer,dest);
      println!("tibemsSession_CreateProducer: {:?}",status);
      let mut msg: usize = 0;
      match message.message_type {
        MessageType::TextMessage =>{
          let status = tibco_ems_sys::tibemsTextMsg_Create(&mut msg);
          println!("tibemsTextMsg_Create: {:?}",status);
          let status = tibco_ems_sys::tibemsTextMsg_SetText(msg,CString::new(message.body_text.clone().unwrap()).unwrap().as_ptr());
          println!("tibemsTextMsg_SetText: {:?}",status);
        },
        MessageType::BytesMessage =>{
          let status = tibco_ems_sys::tibemsBytesMsg_Create(&mut msg);
          println!("tibemsBytesMsg_Create: {:?}",status);
        },
      }
      //set header
      match message.header.clone() {
        Some(headers)=>{
          for (key, val) in &headers {
            let status = tibco_ems_sys::tibemsMsg_SetStringProperty(msg, 
              CString::new(key.to_string()).unwrap().as_ptr(), 
              CString::new(val.to_string()).unwrap().as_ptr());
            println!("tibemsMsg_SetStringProperty: {:?}",status);
          }
        },
        None => {},
      }
      let status = tibco_ems_sys::tibemsMsgProducer_Send(producer, msg);
      println!("tibemsMsgProducer_Send: {:?}",status);
      //destroy message
      let status = tibco_ems_sys::tibemsMsg_Destroy(msg);
      println!("tibemsMsg_Destroy: {:?}",status);
      //destroy producer
      let status = tibco_ems_sys::tibemsMsgProducer_Close(producer);
      println!("tibemsMsgProducer_Close: {:?}",status);
      //destroy destination
      let status = tibco_ems_sys::tibemsDestination_Destroy(dest);
      println!("tibemsDestination_Destroy: {:?}",status);
    }
    Ok(())
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
          println!("tibemsMsg_Destroy: {:?}",status);
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