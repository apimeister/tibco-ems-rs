use std::ffi::CString;
use std::ffi::CStr;
use std::collections::HashMap;
use std::io::Error;

pub mod c_binding;

/// holds the native Connection pointer
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct Connection{
  pointer: usize
}

/// holds the native Session pointer
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct Session{
  pointer: usize
}

/// holds the native Consumer pointer
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct Consumer{
  pointer: usize
}

/// represents a Text Message which can be transformed into Message through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct TextMessage{
  pub body: String,
  pub header: Option<HashMap<String,String>>
}

/// represents a generic Message which can be transformed into TextMessage through From,Into trait.
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Message{
  pub message_type: MessageType,
  body_text: Option<String>,
  body_binary: Option<Vec<u8>>,
}

impl From<Message> for TextMessage {
  fn from(msg: Message) -> Self {
    TextMessage{
      body: msg.body_text.unwrap(),
      header: None,
    }
  }
}

impl From<TextMessage> for Message {
  fn from(msg: TextMessage) -> Self {
    Message{
      message_type: MessageType::TextMessage,
      body_text: Some(msg.body),
      body_binary: None,
    }
  }
}

/// Destination, can either be Queue or Topic
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Destination{
  pub destination_type: DestinationType,
  pub destination_name: String,
}

/// Type of the message
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub enum MessageType{
  TextMessage,
  BytesMessage,
}

/// Type of the destination
#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub enum DestinationType{
  Queue,
  Topic
}

/// open a connection to the Tibco EMS server
pub fn connect(url: String, user: String, password: String) -> Result<Connection, Error> {
  let conn: Connection;
  let mut connection_pointer: usize = 0;
  unsafe{
    let factory = c_binding::tibemsConnectionFactory_Create();
    let status = c_binding::tibemsConnectionFactory_SetServerURL(factory, CString::new(url).unwrap().as_ptr());
    println!("tibemsConnectionFactory_SetServerURL: {:?}",status);
    let status = c_binding::tibemsConnectionFactory_CreateConnection(factory,&mut connection_pointer,CString::new(user).unwrap().as_ptr(),CString::new(password).unwrap().as_ptr());
    println!("tibemsConnectionFactory_CreateConnection: {:?}",status);
    conn = Connection{pointer: connection_pointer};
    let status = c_binding::tibemsConnection_Start(connection_pointer);
    println!("tibemsConnectionFactory_CreateConnection: {:?}",status);
  }
  Ok(conn)
}

/// open a session
pub fn session(connection: Connection)-> Result<Session,Error> {
  let session: Session;
  unsafe{
    let mut session_pointer:usize = 0;
    let status = c_binding::tibemsConnection_CreateSession(connection.pointer, &mut session_pointer, c_binding::tibems_bool::TIBEMS_FALSE, c_binding::tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE);
    println!("tibemsConnection_CreateSession: {:?}",status);
    session = Session{pointer: session_pointer};
  }
  Ok(session)
}

/// open a message consumer
pub fn queue_consumer(session: Session, destination: Destination, selector: Option<String>)-> Result<Consumer,Error> {
  let consumer: Consumer;
  let mut destination_pointer:usize = 0;
  unsafe{
    //create destination
    match destination.destination_type {
      DestinationType::Queue => {
        let status = c_binding::tibemsDestination_Create(&mut destination_pointer, c_binding::tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
        println!("tibemsDestination_Create: {:?}",status);
      },
      DestinationType::Topic => {
        let status = c_binding::tibemsDestination_Create(&mut destination_pointer, c_binding::tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
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
    let status = c_binding::tibemsSession_CreateConsumer(session.pointer, &mut consumer_pointer,destination_pointer, selector_str, c_binding::tibems_bool::TIBEMS_TRUE);
    println!("tibemsSession_CreateConsumer: {:?}",status);
    consumer = Consumer{pointer: consumer_pointer};
  }
  Ok(consumer)
}

/// close a session
pub fn session_close(session: Session){
  unsafe{
    let status = c_binding::tibemsSession_Close(session.pointer);
    println!("tibemsSession_Close: {:?}",status);
  }
}

/// receive messages from a consumer
pub fn receive_message(consumer: Consumer, wait_time_ms: Option<i64>) -> Result<Option<Message>,Error> {
  let mut msg:Message = Message{
    message_type: MessageType::TextMessage,
    body_text: None,
    body_binary: None,
  };
  unsafe{
    let mut msg_pointer:usize = 0;
    match wait_time_ms {
      Some(time_ms) => {
        let status = c_binding::tibemsMsgConsumer_ReceiveTimeout(consumer.pointer, &mut msg_pointer, time_ms);
        println!("tibemsMsgConsumer_Receive: {:?}",status);
        if status == c_binding::tibems_status::TIBEMS_TIMEOUT {
          return Ok(None)
        }
      },
      None => {
        let status = c_binding::tibemsMsgConsumer_Receive(consumer.pointer, &mut msg_pointer);
        println!("tibemsMsgConsumer_Receive: {:?}",status);    
      },
    }
    let mut msg_type: c_binding::tibemsMsgType = c_binding::tibemsMsgType::TIBEMS_TEXT_MESSAGE;
    let status = c_binding::tibemsMsg_GetBodyType(msg_pointer, &mut msg_type);
    println!("tibemsMsg_GetBodyType: {:?}",status);
    match msg_type {
      c_binding::tibemsMsgType::TIBEMS_TEXT_MESSAGE => {
        let buf_vec:Vec<i8> = vec![0; 0];
        let buf_ref: *const std::os::raw::c_char = buf_vec.as_ptr();
        let status = c_binding::tibemsTextMsg_GetText(msg_pointer, & buf_ref);
        println!("tibemsTextMsg_GetText: {:?}",status);
        let content = CStr::from_ptr(buf_ref).to_str().unwrap();
        msg = Message{
          message_type: MessageType::TextMessage,
          body_text: Some(content.to_string()),
          body_binary: None,
        };
      },
      _ => {
        //unknown
        println!("BodyType: {:?}",msg_type);
      }
    }
  }
  Ok(Some(msg))
}
/// sending a message to a destination (only queues are supported)
pub fn send_message(session: Session, destination: Destination, message: Message) -> Result<(),Error>{
  let mut dest:usize = 0;
  unsafe{
    match destination.destination_type {
      DestinationType::Queue => {
        let status = c_binding::tibemsDestination_Create(&mut dest, c_binding::tibemsDestinationType::TIBEMS_QUEUE, CString::new(destination.destination_name).unwrap().as_ptr());
        println!("tibemsDestination_Create: {:?}",status);
      },
      DestinationType::Topic => {
        let status = c_binding::tibemsDestination_Create(&mut dest, c_binding::tibemsDestinationType::TIBEMS_TOPIC, CString::new(destination.destination_name).unwrap().as_ptr());
        println!("tibemsDestination_Create: {:?}",status);
      }
    }
    let mut producer: usize = 0;
    let status = c_binding::tibemsSession_CreateProducer(session.pointer,&mut producer,dest);
    println!("tibemsSession_CreateProducer: {:?}",status);
    let mut msg: usize = 0;
    match message.message_type {
      MessageType::TextMessage =>{
        let status = c_binding::tibemsTextMsg_Create(&mut msg);
        println!("tibemsTextMsg_Create: {:?}",status);    
        let status = c_binding::tibemsTextMsg_SetText(msg,CString::new(message.body_text.unwrap()).unwrap().as_ptr());
        println!("tibemsTextMsg_SetText: {:?}",status);
      }
      _ => {
        let status = c_binding::tibemsTextMsg_Create(&mut msg);
        println!("tibemsTextMsg_Create: {:?}",status);    
      }
    }
    let status = c_binding::tibemsMsgProducer_Send(producer, msg);
    println!("tibemsMsgProducer_Send: {:?}",status);
  }
  Ok(())
}