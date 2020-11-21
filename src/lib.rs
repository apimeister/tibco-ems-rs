use std::ffi::CString;
use std::ffi::CStr;
use std::collections::HashMap;

pub mod c_binding;

#[allow(dead_code)]
pub struct Connection{
  pointer: usize
}

#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct Session{
  pointer: usize
}

#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub struct Consumer{
  pointer: usize
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct TextMessage{
  pub body: String,
  pub header: Option<HashMap<String,String>>
}

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

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Destination{
  pub destination_type: DestinationType,
  pub destination_name: String,
}

#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub enum MessageType{
  TextMessage,
  BytesMessage,
}

#[allow(dead_code)]
#[derive(Debug,Copy,Clone)]
pub enum DestinationType{
  Queue,
  Topic
}
pub fn connect(url: String, user: String, password: String) -> Connection {
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
  conn
}

pub fn session(connection: Connection)-> Session {
  let session: Session;
  unsafe{
    let mut session_pointer:usize = 0;
    let status = c_binding::tibemsConnection_CreateSession(connection.pointer, &mut session_pointer, c_binding::tibems_bool::TIBEMS_FALSE, c_binding::tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE);
    println!("tibemsConnection_CreateSession: {:?}",status);
    session = Session{pointer: session_pointer};
  }
  session
}

pub fn queue_consumer(session: Session, destination: Destination, selector: Option<String>)-> Consumer {
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
    let status = c_binding::tibemsSession_CreateConsumer(session.pointer, &mut consumer_pointer,destination_pointer, std::ptr::null_mut(), c_binding::tibems_bool::TIBEMS_TRUE);
    println!("tibemsSession_CreateConsumer: {:?}",status);
    consumer = Consumer{pointer: consumer_pointer};
  }
  consumer
}

pub fn session_close(session: Session){
  unsafe{
    let status = c_binding::tibemsSession_Close(session.pointer);
    println!("tibemsSession_Close: {:?}",status);
  }
}

pub fn receive_message(consumer: Consumer, wait_time: Option<u64>) -> Message {
  let mut msg:Message = Message{
    message_type: MessageType::TextMessage,
    body_text: None,
    body_binary: None,
  };
  unsafe{
    let mut msg_pointer:usize = 0;
    let status = c_binding::tibemsMsgConsumer_Receive(consumer.pointer, &mut msg_pointer);
    println!("tibemsMsgConsumer_Receive: {:?}",status);
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
  msg
}
pub fn send_text_message(session: Session, destination: Destination, message: TextMessage){
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
    let status = c_binding::tibemsTextMsg_Create(&mut msg);
    println!("tibemsTextMsg_Create: {:?}",status);
    let status = c_binding::tibemsTextMsg_SetText(msg,CString::new(message.body).unwrap().as_ptr());
    println!("tibemsTextMsg_SetText: {:?}",status);
    let status = c_binding::tibemsMsgProducer_Send(producer, msg);
    println!("tibemsMsgProducer_Send: {:?}",status);
  }
}