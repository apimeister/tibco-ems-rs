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
use log::{trace, error, warn};
use serde::{Serialize, Deserialize};

const ADMIN_QUEUE: &str = "$sys.admin";

/// open a connection to the Tibco EMS server for administrative purposes
pub fn connect(url: &str, user: &str, password: &str) -> Result<Connection, Error> {
  let conn =  super::connect(url,user,password);
  match conn {
    Ok(conn)=>{
      //check connection for active server
      let active_url = conn.get_active_url().unwrap();
      drop(conn);
      let admin_active_url = format!("<$admin>:{}",active_url);
      let conn2 =  super::connect(&admin_active_url,user,password);
      conn2
    },
    Err(err)=> Err(err)
  }
}

//
// Queues
// 

/// holds static queue information
#[derive(Debug, Clone, Default,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueInfo{
  /// name of the queue
  pub name: String,
  /// pending messages
  pub pending_messages: Option<i64>,
  /// max allowed messages
  pub max_messages: Option<i64>,
  /// max size
  pub max_bytes: Option<i64>,
  /// overflow policy
  pub overflow_policy: Option<OverflowPolicy>,
  /// failsafe
  pub failsafe: Option<bool>,
  /// secure
  pub secure: Option<bool>,
  /// global
  pub global: Option<bool>,
  /// sender name
  pub sender_name: Option<bool>,
  /// sender name enforced
  pub sender_name_enforced: Option<bool>,
  /// prefetch
  pub prefetch: Option<i32>,
  /// expiration override
  pub expiry_override: Option<i64>,
  /// redelivery delay
  pub redelivery_delay: Option<i64>,
  /// count of comsumers
  pub consumer_count: Option<i32>,
}

/// lists all queues present on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn list_all_queues(session: &Session) -> Vec<QueueInfo> {
  let mut queues = Vec::new();
  const TIMEOUT: i64 = 60000;
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dt".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
  msg.body.insert("permType".to_string(), TypedValue::int_value(6));
  msg.body.insert("pattern".to_string(), TypedValue::string_value(">".to_string()));
  msg.body.insert("ia".to_string(), TypedValue::bool_value(true));
  msg.body.insert("first".to_string(), TypedValue::int_value(1000));
  
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::ListDestination as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));
  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
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
                let consumer_count = q_info.body.get("cc").unwrap().string_value().unwrap();
                let expiry = q_info.body.get("expy").unwrap().string_value().unwrap();
                let redelivery_delay = q_info.body.get("rdd").unwrap().string_value().unwrap();
                                    
                let queue_info = QueueInfo{
                  name: key,
                  pending_messages: Some(pending_messages.parse::<i64>().unwrap()),
                  max_messages: Some(max_msgs.parse::<i64>().unwrap()),
                  max_bytes: Some(max_bytes.parse::<i64>().unwrap()),
                  overflow_policy: Some(overflow_policy),
                  failsafe: Some(bool_failsafe),
                  secure: Some(bool_secure),
                  global: Some(bool_global),
                  sender_name: Some(bool_sender_name),
                  sender_name_enforced: Some(bool_sn_enforced),
                  prefetch: Some(prefetch.parse::<i32>().unwrap()),
                  expiry_override: Some(expiry.parse::<i64>().unwrap()),
                  redelivery_delay: Some(redelivery_delay.parse::<i64>().unwrap()),
                  consumer_count: Some(consumer_count.parse::<i32>().unwrap()),
                };
                queues.push(queue_info);
              }
            },
            _ =>{
              warn!("unkown response from queue information request")
            }
          }
        },
        None=>{},
      }
    },
    Err(err) =>{
      error!("something went wronge retrieving queue information: {}",err);
    }
  }
  queues
}

/// creates a queue on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn create_queue(session: &Session, queue: &QueueInfo){
  //create queue map-message
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dn".to_string(), TypedValue::string_value(queue.name.clone()));
  msg.body.insert("dt".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
  match queue.max_bytes {
    Some(val) => {
      msg.body.insert("mb".to_string(), TypedValue::long_value(val));
    },
    _ => {},
  }
  match queue.max_messages {
    Some(val) => {
      msg.body.insert("mm".to_string(), TypedValue::long_value(val));
    },
    _ => {},
  }
  match queue.global {
    Some(val) => {
      msg.body.insert("global".to_string(), TypedValue::bool_value(val));
    },
    _ => {},
  }

  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("JMS_TIBCO_MSG_EXT".to_string(),TypedValue::bool_value(true));
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::CreateDestination as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));

  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
  };
  let result = session.send_message(dest, msg.into());
  match result {
    Ok(_) => {},
    Err(err) => {
      error!("error while creating queue {}: {}",queue.name,err);
    }
  }
}

/// deletes a queue from the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn delete_queue(session: &Session, queue: &str){
  trace!("deleting queue {}", queue);
  //create queue map-message
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dn".to_string(), TypedValue::string_value(queue.to_string()));
  msg.body.insert("dt".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("JMS_TIBCO_MSG_EXT".to_string(),TypedValue::bool_value(true));
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::DeleteDestination as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));

  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
  };
  let result = session.send_message(dest, msg.into());
  match result {
    Ok(_) => {},
    Err(err) => {
      error!("error while deleting queue {}: {}",queue,err);
    }
  }
}

//
// Topics
// 

/// holds static topic information
#[derive(Debug, Clone, Default,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfo{
  /// name of the topic
  pub name: String,
  /// expiration override
  pub expiry_override: Option<i64>,
  /// global
  pub global: Option<bool>,
  /// max size
  pub max_bytes: Option<i64>,
  /// max number of messages
  pub max_messages: Option<i64>,
  /// overflow policy
  pub overflow_policy: Option<OverflowPolicy>,
  /// prefetch
  pub prefetch: Option<i32>,
  /// count of durables
  pub durable_count: Option<i32>,
  /// count of subscribers
  pub subscriber_count: Option<i32>,
  /// count of pending messages
  pub pending_messages: Option<i64>,
}

/// lists all topics present on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn list_all_topics(session: &Session) -> Vec<TopicInfo> {
  let mut topics = Vec::new();
  const TIMEOUT: i64 = 60000;
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dt".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
  msg.body.insert("permType".to_string(), TypedValue::int_value(6));
  msg.body.insert("pattern".to_string(), TypedValue::string_value(">".to_string()));
  msg.body.insert("ia".to_string(), TypedValue::bool_value(true));
  msg.body.insert("first".to_string(), TypedValue::int_value(1000));
  
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::ListDestination as i32));
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
                let durable_count = t_info.body.get("cd").unwrap().string_value().unwrap();
                let subscriber_count = t_info.body.get("sc").unwrap().string_value().unwrap();
                let pending_messages = t_info.body.get("nm").unwrap().string_value().unwrap();
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
                  expiry_override: Some(expiry.parse::<i64>().unwrap()),
                  global: Some(bool_global),
                  max_bytes: Some(max_bytes.parse::<i64>().unwrap()),
                  max_messages: Some(max_msgs.parse::<i64>().unwrap()),
                  overflow_policy: Some(overflow_policy),
                  prefetch: Some(prefetch.parse::<i32>().unwrap()),
                  durable_count: Some(durable_count.parse::<i32>().unwrap()),
                  subscriber_count: Some(subscriber_count.parse::<i32>().unwrap()),
                  pending_messages: Some(pending_messages.parse::<i64>().unwrap()),
                };
                topics.push(topic_info);
              }
            },
            _ =>{
              warn!("unkown response from topic information request")
            }
          }
        },
        None=>{},
      }
    },
    Err(err) =>{
      error!("something went wrong retrieving topic information: {}",err);
    }
  }
  topics
}

/// creates a topic on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn create_topic(session: &Session, topic: &TopicInfo){
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dn".to_string(), TypedValue::string_value(topic.name.clone()));
  msg.body.insert("dt".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
  match topic.max_bytes {
    Some(val) => {
      msg.body.insert("mb".to_string(), TypedValue::long_value(val));
    },
    _ => {},
  }
  match topic.max_messages {
    Some(val) => {
      msg.body.insert("mm".to_string(), TypedValue::long_value(val));
    },
    _ => {},
  }
  match topic.global {
    Some(val) => {
      msg.body.insert("global".to_string(), TypedValue::bool_value(val));
    },
    _ => {},
  }

  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("JMS_TIBCO_MSG_EXT".to_string(),TypedValue::bool_value(true));
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::CreateDestination as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));

  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
  };
  let result = session.send_message(dest, msg.into());
  match result {
    Ok(_) => {},
    Err(err) => {
      error!("error while creating topic {}: {}",topic.name,err);
    }
  }
}

/// deletes a topic from the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn delete_topic(session: &Session, topic: &str) {
  trace!("deleting topic {}", topic);
  //create topic map-message
  let mut msg: MapMessage = Default::default();
  msg.body.insert("dn".to_string(), TypedValue::string_value(topic.to_string()));
  msg.body.insert("dt".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("JMS_TIBCO_MSG_EXT".to_string(),TypedValue::bool_value(true));
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::DeleteDestination as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));

  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
  };
  let result = session.send_message(dest, msg.into());
  match result {
    Ok(_) => {},
    Err(err) => {
      error!("error while deleting topic {}: {}",topic,err);
    }
  }
}

//
// Bridges
// 

/// create a bridge
pub fn create_bridge(session: &Session, bridge: &BridgeInfo) {
  //create bridge map-message
  let source_name = bridge.source_name.clone();
  let target_name = bridge.target_name.clone();
  let mut msg: MapMessage = Default::default();
  msg.body.insert("sn".to_string(), TypedValue::string_value(source_name));
  match bridge.source_type {
    DestinationType::Queue =>{
      msg.body.insert("st".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
    },
    DestinationType::Topic =>{
      msg.body.insert("st".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
    },
  }

  msg.body.insert("tn".to_string(), TypedValue::string_value(target_name));
  match bridge.target_type {
    DestinationType::Queue =>{
      msg.body.insert("tt".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
    },
    DestinationType::Topic =>{
      msg.body.insert("tt".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
    },
  }
  match bridge.selector.clone() {
    Some(sel) => {
      msg.body.insert("sel".to_string(), TypedValue::string_value(sel));
    },
    None => {},
  }
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("JMS_TIBCO_MSG_EXT".to_string(),TypedValue::bool_value(true));
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::CreateBridge as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));

  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
  };
  let result = session.send_message(dest, msg.into());
  match result {
    Ok(_) => {},
    Err(err) => {
      error!("error while creating bridge {}->{}: {}",bridge.source_name,bridge.target_name,err);
    },
  }
}

/// delete a bridge
pub fn delete_bridge(session: &Session, bridge: &BridgeInfo) {
  let source_name = bridge.source_name.clone();
  let target_name = bridge.target_name.clone();

  //create bridge map-message
  let mut msg: MapMessage = Default::default();
  msg.body.insert("sn".to_string(), TypedValue::string_value(source_name));
  match bridge.source_type {
    DestinationType::Queue =>{
      msg.body.insert("st".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
    },
    DestinationType::Topic =>{
      msg.body.insert("st".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
    },
  }

  msg.body.insert("tn".to_string(), TypedValue::string_value(target_name));
  match bridge.target_type {
    DestinationType::Queue =>{
      msg.body.insert("tt".to_string(), TypedValue::int_value(DestinationType::Queue as i32));
    },
    DestinationType::Topic =>{
      msg.body.insert("tt".to_string(), TypedValue::int_value(DestinationType::Topic as i32));
    },
  }
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("JMS_TIBCO_MSG_EXT".to_string(),TypedValue::bool_value(true));
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::DeleteBrdige as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));
  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
  };
  let result = session.send_message(dest, msg.into());
  match result {
    Ok(_) => {},
    Err(err) => {
      error!("error while deleting bridge {}->{}: {}",bridge.source_name,bridge.target_name,err);
    }
  } 
}

//
// Server
//

/// get server state
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn get_server_state(session: &Session) -> ServerState {
  const TIMEOUT: i64 = 60000;
  let mut msg: MapMessage = Default::default();
  
  //header
  let mut header: HashMap<String,TypedValue> = HashMap::new();
  //actual boolean
  header.insert("code".to_string(),TypedValue::int_value(AdminCommands::GetStateInfo as i32));
  header.insert("save".to_string(),TypedValue::bool_value(true));
  header.insert("arseq".to_string(),TypedValue::int_value(1));
  msg.header = Some(header);

  let dest = Destination{
    destination_type: DestinationType::Queue,
    destination_name: ADMIN_QUEUE.to_string(),
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
              let state_str = map_message.body.get("state").unwrap().string_value().unwrap();
              if state_str == "3"{
                return ServerState::Standby;
              }else{
                return ServerState::Active;
              }
            },
            _ =>{
              warn!("unkown response from queue information request")
            }
          }
        },
        None=>{},
      }
    },
    Err(err) =>{
      error!("something went wronge retrieving queue information: {}",err);
    }
  }
  return ServerState::Active;
}

/// holds static bridge information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeInfo{
  /// type of the bridge source
  pub source_type: DestinationType,
  /// name of the bridge source
  pub source_name: String,
  /// type of the bridge target
  pub target_type: DestinationType,
  /// name of the bridge target
  pub target_name: String,
  /// selector
  pub selector: Option<String>,
}

/// available overflow policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverflowPolicy{
  /// default overflow policy
  Default = 0,
  /// discard old message if destination overflows
  DiscardOld = 1,
  /// reject incoming message if destination overflows
  RejectIncoming = 2,
}

/// admin command codes used on the admin queue
#[derive(Debug,Clone)]
pub enum AdminCommands{
  /// delete a destination
  DeleteDestination = 16,
  /// create a destination
  CreateDestination = 18,
  /// list destinations
  ListDestination = 19,
  /// get server info
  GetServerInfo = 120,
  /// get state info
  GetStateInfo = 127,
  /// create a bridge
  CreateBridge = 220,
  /// delete a bridge
  DeleteBrdige = 221,
}

/// server states
#[derive(Debug,Clone)]
pub enum ServerState{
  // server is standby
  Standby = 3,
  // server is active
  Active = 4,
}