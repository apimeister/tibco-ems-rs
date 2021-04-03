//! Tibco EMS admin functions.

use std::io::Error;
use super::Connection;
use super::Destination;
use super::DestinationType;
use super::TypedValue;
use super::MapMessage;
use super::MessageType;
use super::GetMapValue;
use super::GetStringValue;
use super::Session;
use std::collections::HashMap;

/// open a connection to the Tibco EMS server for administrative purposes
pub fn connect(url: &str, user: &str, password: &str) -> Result<Connection, Error> {
  let admin_url = format!("<$admin>:{}",url);
  return super::connect(&admin_url,user,password);
}

pub fn list_all_queues(session: &Session) -> Vec<QueueInfo> {
  let mut queues = Vec::new();
  const TIMEOUT: i64 = 60000;
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dt".to_string(), TypedValue::int_value(1));
  msg.body.insert("permType".to_string(), TypedValue::int_value(6));
  msg.body.insert("pattern".to_string(), TypedValue::string_value(">".to_string()));
  msg.body.insert("ia".to_string(), TypedValue::bool_value(true));
  msg.body.insert("first".to_string(), TypedValue::int_value(1000));
  
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("code".to_string(),TypedValue::int_value(19));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));
  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: "$sys.admin".to_string(),
  };
  let query_result = session.request_reply(dest, msg.into(), TIMEOUT);
  match query_result{
    Ok(response) =>{
      match response{
        Some(resp)=>{
          match resp.message_type {
            MessageType::MapMessage => {
              //got response message
              let map_message: MapMessage = resp.into();
              for (key,val) in map_message.body {
                let q_info: MapMessage = val.map_value().unwrap();
                let pending_messages = q_info.body.get("nm").unwrap().string_value().unwrap();
                let max_bytes = q_info.body.get("mb").unwrap().string_value().unwrap();
                let max_msgs = q_info.body.get("mm").unwrap().string_value().unwrap();
                let overflow = q_info.body.get("op").unwrap().string_value().unwrap();
                let overflow_policy: OverflowPolicy;
                match overflow.as_str() {
                  "0"=> overflow_policy=OverflowPolicy::Default,
                  "1"=> overflow_policy=OverflowPolicy::DiscardOld,
                  "2"=> overflow_policy=OverflowPolicy::RejectIncoming,
                  _ => overflow_policy=OverflowPolicy::Default,
                }
                let mut bool_failsafe = false;
                match q_info.body.get("failsafe") {
                  Some(val) => {
                    let failsafe = val.string_value().unwrap();
                    bool_failsafe = failsafe == "1";
                  },
                  None =>{},
                }
                let mut bool_secure = false;
                match q_info.body.get("secure") {
                  Some(val) => {
                    let secure = val.string_value().unwrap();
                    bool_secure = secure == "1";
                  },
                  None =>{},
                }
                let mut bool_global = false;
                match q_info.body.get("global") {
                  Some(val) => {
                    let global = val.string_value().unwrap();
                    bool_global = global == "1";    
                  },
                  None =>{},
                }
                let mut bool_sender_name = false;
                match q_info.body.get("sname") {
                  Some(val) => {
                    let sender_name = val.string_value().unwrap();
                    bool_sender_name = sender_name == "1";
                  },
                  None =>{},
                }
                let mut bool_sn_enforced = false;
                match q_info.body.get("snameenf") {
                  Some(val) => {
                    let sn_enforced = val.string_value().unwrap();
                    bool_sn_enforced = sn_enforced == "1";    
                  },
                  None =>{},
                }
                let prefetch = q_info.body.get("pf").unwrap().string_value().unwrap();
                let expiry = q_info.body.get("expy").unwrap().string_value().unwrap();
                let redelivery_delay = q_info.body.get("rdd").unwrap().string_value().unwrap();
                                    
                let queue_info = QueueInfo{
                  name: key,
                  pending_messages: pending_messages.parse::<i64>().unwrap(),
                  max_messages: max_msgs.parse::<i64>().unwrap(),
                  max_bytes: max_bytes.parse::<i64>().unwrap(),
                  overflow_policy: overflow_policy,
                  failsafe: bool_failsafe,
                  secure: bool_secure,
                  global: bool_global,
                  sender_name: bool_sender_name,
                  sender_name_enforced: bool_sn_enforced,
                  prefetch: prefetch.parse::<i32>().unwrap(),
                  expiry_override: expiry.parse::<i64>().unwrap(),
                  redelivery_delay: redelivery_delay.parse::<i64>().unwrap(),
                };
                queues.push(queue_info);
              }
            },
            _ =>{
              println!("unkown response from queue information request")
            }
          }
        },
        None=>{},
      }
    },
    Err(err) =>{
      println!("something went wronge retrieving queue information: {}",err);
    }
  }
  queues
}

pub fn list_all_topics(session: &Session) -> Vec<TopicInfo> {
  let mut topics = Vec::new();
  const TIMEOUT: i64 = 60000;
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dt".to_string(), TypedValue::int_value(2));
  msg.body.insert("permType".to_string(), TypedValue::int_value(6));
  msg.body.insert("pattern".to_string(), TypedValue::string_value(">".to_string()));
  msg.body.insert("ia".to_string(), TypedValue::bool_value(true));
  msg.body.insert("first".to_string(), TypedValue::int_value(1000));
  
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("code".to_string(),TypedValue::int_value(19));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));
  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: "$sys.admin".to_string(),
  };
  let query_result = session.request_reply(dest, msg.into(), TIMEOUT);
  match query_result{
    Ok(response) =>{
      match response{
        Some(resp)=>{
          match resp.message_type {
            MessageType::MapMessage => {
              //got response message
              let map_message: MapMessage = resp.into();
              for (key,val) in map_message.body {
                let t_info: MapMessage = val.map_value().unwrap();
                let mut bool_global = false;
                match t_info.body.get("global") {
                  Some(val) => {
                    let global = val.string_value().unwrap();
                    bool_global = global == "1";    
                  },
                  None =>{},
                }
                let prefetch = t_info.body.get("pf").unwrap().string_value().unwrap();
                let expiry = t_info.body.get("expy").unwrap().string_value().unwrap();
                let max_bytes = t_info.body.get("mb").unwrap().string_value().unwrap();
                let max_msgs = t_info.body.get("mm").unwrap().string_value().unwrap();
                let overflow = t_info.body.get("op").unwrap().string_value().unwrap();
                let overflow_policy: OverflowPolicy;
                match overflow.as_str() {
                  "0"=> overflow_policy=OverflowPolicy::Default,
                  "1"=> overflow_policy=OverflowPolicy::DiscardOld,
                  "2"=> overflow_policy=OverflowPolicy::RejectIncoming,
                  _ => overflow_policy=OverflowPolicy::Default,
                }
                
                let topic_info = TopicInfo{
                  name: key,
                  expiry_override: expiry.parse::<i64>().unwrap(),
                  global: bool_global,
                  max_bytes: max_bytes.parse::<i64>().unwrap(),
                  max_messages: max_msgs.parse::<i64>().unwrap(),
                  overflow_policy: overflow_policy,
                  prefetch: prefetch.parse::<i32>().unwrap(),                
                };
                topics.push(topic_info);
              }
            },
            _ =>{
              println!("unkown response from topic information request")
            }
          }
        },
        None=>{},
      }
    },
    Err(err) =>{
      println!("something went wronge retrieving topic information: {}",err);
    }
  }
  topics
}

#[derive(Debug,Clone)]
pub struct QueueInfo{
  name: String,
  pending_messages: i64,
  max_messages: i64,
  max_bytes: i64,
  overflow_policy: OverflowPolicy,
  failsafe: bool,
  secure: bool,
  global: bool,
  sender_name: bool,
  sender_name_enforced: bool,
  prefetch: i32,
  expiry_override: i64,
  redelivery_delay: i64,
}

#[derive(Debug,Clone)]
pub struct TopicInfo{
  name: String,
  expiry_override: i64,
  global: bool,
  max_bytes: i64,
  max_messages: i64,
  overflow_policy: OverflowPolicy,
  prefetch: i32,
}

#[derive(Debug,Clone)]
pub enum OverflowPolicy{
  Default = 0,
  DiscardOld = 1,
  RejectIncoming = 2,
}