use tibco_ems::c_binding::*;
use std::ffi::CString;

fn main() {
  unsafe{
    let factory = tibemsConnectionFactory_Create();

    let url = "tcp://localhost:7222";
    let user="admin";
    let password="admin";

    let status = tibemsConnectionFactory_SetServerURL(factory, CString::new(url).unwrap().as_ptr());
    println!("tibemsConnectionFactory_SetServerURL: {:?}",status);

    let mut conn: usize = 0;
    let status = tibemsConnectionFactory_CreateConnection(factory,&mut conn,CString::new(user).unwrap().as_ptr(),CString::new(password).unwrap().as_ptr());
    println!("tibemsConnectionFactory_CreateConnection: {:?}",status);

    let mut sess: usize = 0;
    let status = tibemsConnection_CreateSession(conn,&mut sess,tibems_bool::TIBEMS_FALSE,tibemsAcknowledgeMode::TIBEMS_AUTO_ACKNOWLEDGE);
    println!("tibemsConnection_CreateSession: {:?}",status);

    let dest_str = "myqueue";
    let mut dest: usize = 0;
    let status = tibemsDestination_Create(&mut dest, tibemsDestinationType::TIBEMS_QUEUE, CString::new(dest_str).unwrap().as_ptr());
    println!("tibemsDestination_Create: {:?}",status);

    let mut producer: usize = 0;
    let status = tibemsSession_CreateProducer(sess,&mut producer,dest);
    println!("tibemsSession_CreateProducer: {:?}",status);

    let mut msg: usize = 0;
    let status = tibemsTextMsg_Create(&mut msg);
    println!("tibemsTextMsg_Create: {:?}",status);

    let content="hallo welt";
    let status = tibemsTextMsg_SetText(msg,CString::new(content).unwrap().as_ptr());
    println!("tibemsTextMsg_SetText: {:?}",status);

    let status = tibemsMsg_SetStringProperty(msg,CString::new("key").unwrap().as_ptr(),CString::new("val").unwrap().as_ptr());
    println!("tibemsMsg_SetStringProperty: {:?}",status);

    let status = tibemsMsgProducer_Send(producer, msg);
    println!("tibemsMsgProducer_Send: {:?}",status);
  }
}
