//! Tibco EMS admin functions.

use super::Connection;
use super::Destination;
use super::MapMessage;
use super::Message;
use super::Session;
use super::TypedValue;
use enum_extract::extract;
use log::{error, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;

const ADMIN_QUEUE_NAME: &str = "$sys.admin";
const DESTINATION_TYPE_QUEUE: i32 = 1;
const DESTINATION_TYPE_TOPIC: i32 = 2;

/// open a connection to the Tibco EMS server for administrative purposes
pub fn connect(url: &str, user: &str, password: &str) -> Result<Connection, Error> {
    let conn = super::connect(url, user, password);
    match conn {
        Ok(conn) => {
            //check connection for active server
            let active_url = conn.get_active_url().unwrap();
            drop(conn);
            let admin_active_url = format!("<$admin>:{}", active_url);
            super::connect(&admin_active_url, user, password)
        }
        Err(err) => Err(err),
    }
}

//
// Queues
//

/// holds static queue information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueInfo {
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
    /// total count of incoming messages
    pub incoming_total_count: Option<i64>,
    /// total count of outgoing messages
    pub outgoing_total_count: Option<i64>,
}

/// lists all queues present on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn list_all_queues(session: &Session) -> Result<Vec<QueueInfo>, Error> {
    let mut queues = Vec::new();
    const TIMEOUT: i64 = 60000;
    let mut msg: MapMessage = Default::default();
    msg.body.insert(
        "dt".to_string(),
        TypedValue::Integer(DESTINATION_TYPE_QUEUE),
    );
    msg.body
        .insert("permType".to_string(), TypedValue::Integer(6));
    msg.body
        .insert("pattern".to_string(), TypedValue::String(">".to_string()));
    msg.body.insert("ia".to_string(), TypedValue::Boolean(true));
    msg.body
        .insert("first".to_string(), TypedValue::Integer(1000));

    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::ListDestination as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let query_result = session.request_reply(&admin_queue, msg, TIMEOUT);
    match query_result {
        Ok(response) => {
            if let Some(resp) = response {
                match &resp {
                    Message::MapMessage(map_message) => {
                        //got response message
                        for (key, val) in &map_message.body {
                            let q_info: &MapMessage =
                                extract!(TypedValue::Map(_), val).expect("extract inner message");
                            let pending_messages =
                                extract!(TypedValue::String(_), q_info.body.get("nm").unwrap())
                                    .expect("extract pending messages");
                            let max_bytes =
                                extract!(TypedValue::String(_), q_info.body.get("mb").unwrap())
                                    .expect("extract max bytes");
                            let max_msgs =
                                extract!(TypedValue::String(_), q_info.body.get("mm").unwrap())
                                    .expect("extract max messages");
                            let overflow =
                                extract!(TypedValue::String(_), q_info.body.get("op").unwrap())
                                    .expect("extract overflow");
                            let overflow_policy: OverflowPolicy = match overflow.as_str() {
                                "0" => OverflowPolicy::Default,
                                "1" => OverflowPolicy::DiscardOld,
                                "2" => OverflowPolicy::RejectIncoming,
                                _ => OverflowPolicy::Default,
                            };
                            let mut bool_failsafe = false;
                            if let Some(val) = q_info.body.get("failsafe") {
                                let failsafe =
                                    extract!(TypedValue::String(_), val).expect("extract failsafe");
                                bool_failsafe = failsafe == "1";
                            }
                            let mut bool_secure = false;
                            if let Some(val) = q_info.body.get("secure") {
                                let secure = extract!(TypedValue::String(_), val)
                                    .expect("extract secure flag");
                                bool_secure = secure == "1";
                            }
                            let mut bool_global = false;
                            if let Some(val) = q_info.body.get("global") {
                                let global = extract!(TypedValue::String(_), val)
                                    .expect("extract global flag");
                                bool_global = global == "1";
                            }
                            let mut bool_sender_name = false;
                            if let Some(val) = q_info.body.get("sname") {
                                let sender_name = extract!(TypedValue::String(_), val)
                                    .expect("extract sender name");
                                bool_sender_name = sender_name == "1";
                            }
                            let mut bool_sn_enforced = false;
                            if let Some(val) = q_info.body.get("snameenf") {
                                let sn_enforced = extract!(TypedValue::String(_), val)
                                    .expect("extrance sender name enforced");
                                bool_sn_enforced = sn_enforced == "1";
                            }
                            let prefetch =
                                extract!(TypedValue::String(_), q_info.body.get("pf").unwrap())
                                    .expect("queue property");
                            let consumer_count =
                                extract!(TypedValue::String(_), q_info.body.get("cc").unwrap())
                                    .expect("queue property");
                            let expiry =
                                extract!(TypedValue::String(_), q_info.body.get("expy").unwrap())
                                    .expect("queue property");
                            let redelivery_delay =
                                extract!(TypedValue::String(_), q_info.body.get("rdd").unwrap())
                                    .expect("queue property");
                            let in_total_count =
                                extract!(TypedValue::String(_), q_info.body.get("inct").unwrap())
                                    .expect("queue property");
                            let out_total_count =
                                extract!(TypedValue::String(_), q_info.body.get("outct").unwrap())
                                    .expect("queue property");

                            let queue_info = QueueInfo {
                                name: key.to_string(),
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
                                incoming_total_count: Some(in_total_count.parse::<i64>().unwrap()),
                                outgoing_total_count: Some(out_total_count.parse::<i64>().unwrap()),
                            };
                            queues.push(queue_info);
                        }
                    }
                    _ => {
                        warn!("unkown response from queue information request")
                    }
                }
            }
        }
        Err(err) => {
            error!(
                "something went wronge retrieving queue information: {}",
                err
            );
            return Err(err);
        }
    }
    Ok(queues)
}

/// creates a queue on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn create_queue(session: &Session, queue: &QueueInfo) -> Result<(), Error> {
    //create queue map-message
    let mut msg: MapMessage = Default::default();
    msg.body
        .insert("dn".to_string(), TypedValue::String(queue.name.clone()));
    msg.body.insert(
        "dt".to_string(),
        TypedValue::Integer(DESTINATION_TYPE_QUEUE),
    );
    if let Some(val) = queue.max_bytes {
        msg.body.insert("mb".to_string(), TypedValue::Long(val));
    }
    if let Some(val) = queue.max_messages {
        msg.body.insert("mm".to_string(), TypedValue::Long(val));
    }
    if let Some(val) = queue.global {
        msg.body
            .insert("global".to_string(), TypedValue::Boolean(val));
    }
    if let Some(val) = queue.prefetch {
        msg.body.insert("pf".to_string(), TypedValue::Integer(val));
    }

    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert("JMS_TIBCO_MSG_EXT".to_string(), TypedValue::Boolean(true));
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::CreateDestination as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let result = session.send_message(&admin_queue, msg);
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("error while creating queue {}: {}", queue.name, err);
            return Err(err);
        }
    }
    Ok(())
}

/// deletes a queue from the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn delete_queue(session: &Session, queue: &str) -> Result<(), Error> {
    trace!("deleting queue {}", queue);
    //create queue map-message
    let mut msg: MapMessage = Default::default();
    msg.body
        .insert("dn".to_string(), TypedValue::String(queue.to_string()));
    msg.body.insert(
        "dt".to_string(),
        TypedValue::Integer(DESTINATION_TYPE_QUEUE),
    );
    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert("JMS_TIBCO_MSG_EXT".to_string(), TypedValue::Boolean(true));
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::DeleteDestination as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let result = session.send_message(&admin_queue, msg);
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("error while deleting queue {}: {}", queue, err);
            return Err(err);
        }
    }
    Ok(())
}

//
// Topics
//

/// holds static topic information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicInfo {
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
    /// total count of incoming messages
    pub incoming_total_count: Option<i64>,
    /// total count of outgoing messages
    pub outgoing_total_count: Option<i64>,
}

/// lists all topics present on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn list_all_topics(session: &Session) -> Result<Vec<TopicInfo>, Error> {
    let mut topics = Vec::new();
    const TIMEOUT: i64 = 60000;
    let mut msg: MapMessage = Default::default();
    msg.body.insert(
        "dt".to_string(),
        TypedValue::Integer(DESTINATION_TYPE_TOPIC),
    );
    msg.body
        .insert("permType".to_string(), TypedValue::Integer(6));
    msg.body
        .insert("pattern".to_string(), TypedValue::String(">".to_string()));
    msg.body.insert("ia".to_string(), TypedValue::Boolean(true));
    msg.body
        .insert("first".to_string(), TypedValue::Integer(1000));

    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::ListDestination as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let query_result = session.request_reply(&admin_queue, msg, TIMEOUT);
    match query_result {
        Ok(response) => {
            if let Some(resp) = response {
                match &resp {
                    Message::MapMessage(map_message) => {
                        //got response message
                        for (key, val) in &map_message.body {
                            let t_info: &MapMessage =
                                extract!(TypedValue::Map(_), val).expect("inner message");
                            let mut bool_global = false;
                            if let Some(val) = t_info.body.get("global") {
                                let global =
                                    extract!(TypedValue::String(_), val).expect("global flag");
                                bool_global = global == "1";
                            }
                            let prefetch =
                                extract!(TypedValue::String(_), t_info.body.get("pf").unwrap())
                                    .expect("extract property");
                            let expiry =
                                extract!(TypedValue::String(_), t_info.body.get("expy").unwrap())
                                    .expect("extract property");
                            let max_bytes =
                                extract!(TypedValue::String(_), t_info.body.get("mb").unwrap())
                                    .expect("extract property");
                            let max_msgs =
                                extract!(TypedValue::String(_), t_info.body.get("mm").unwrap())
                                    .expect("extract property");
                            let durable_count =
                                extract!(TypedValue::String(_), t_info.body.get("cd").unwrap())
                                    .expect("extract property");
                            let subscriber_count =
                                extract!(TypedValue::String(_), t_info.body.get("sc").unwrap())
                                    .expect("extract property");
                            let pending_messages =
                                extract!(TypedValue::String(_), t_info.body.get("nm").unwrap())
                                    .expect("extract property");
                            let in_total_count =
                                extract!(TypedValue::String(_), t_info.body.get("inct").unwrap())
                                    .expect("extract property");
                            let out_total_count =
                                extract!(TypedValue::String(_), t_info.body.get("outct").unwrap())
                                    .expect("extract property");
                            let overflow =
                                extract!(TypedValue::String(_), t_info.body.get("op").unwrap())
                                    .expect("extract property");
                            let overflow_policy: OverflowPolicy = match overflow.as_str() {
                                "0" => OverflowPolicy::Default,
                                "1" => OverflowPolicy::DiscardOld,
                                "2" => OverflowPolicy::RejectIncoming,
                                _ => OverflowPolicy::Default,
                            };

                            let topic_info = TopicInfo {
                                name: key.to_string(),
                                expiry_override: Some(expiry.parse::<i64>().unwrap()),
                                global: Some(bool_global),
                                max_bytes: Some(max_bytes.parse::<i64>().unwrap()),
                                max_messages: Some(max_msgs.parse::<i64>().unwrap()),
                                overflow_policy: Some(overflow_policy),
                                prefetch: Some(prefetch.parse::<i32>().unwrap()),
                                durable_count: Some(durable_count.parse::<i32>().unwrap()),
                                subscriber_count: Some(subscriber_count.parse::<i32>().unwrap()),
                                pending_messages: Some(pending_messages.parse::<i64>().unwrap()),
                                incoming_total_count: Some(in_total_count.parse::<i64>().unwrap()),
                                outgoing_total_count: Some(out_total_count.parse::<i64>().unwrap()),
                            };
                            topics.push(topic_info);
                        }
                    }
                    _ => {
                        warn!("unkown response from topic information request")
                    }
                }
            }
        }
        Err(err) => {
            error!("something went wrong retrieving topic information: {}", err);
            return Err(err);
        }
    }
    Ok(topics)
}

/// creates a topic on the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn create_topic(session: &Session, topic: &TopicInfo) -> Result<(), Error> {
    let mut msg: MapMessage = Default::default();
    msg.body
        .insert("dn".to_string(), TypedValue::String(topic.name.clone()));
    msg.body.insert(
        "dt".to_string(),
        TypedValue::Integer(DESTINATION_TYPE_TOPIC),
    );
    if let Some(val) = topic.max_bytes {
        msg.body.insert("mb".to_string(), TypedValue::Long(val));
    }
    if let Some(val) = topic.max_messages {
        msg.body.insert("mm".to_string(), TypedValue::Long(val));
    }
    if let Some(val) = topic.global {
        msg.body
            .insert("global".to_string(), TypedValue::Boolean(val));
    }
    if let Some(val) = topic.prefetch {
        msg.body.insert("pf".to_string(), TypedValue::Integer(val));
    }

    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert("JMS_TIBCO_MSG_EXT".to_string(), TypedValue::Boolean(true));
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::CreateDestination as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let result = session.send_message(&admin_queue, msg);
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("error while creating topic {}: {}", topic.name, err);
            return Err(err);
        }
    }
    Ok(())
}

/// deletes a topic from the EMS
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn delete_topic(session: &Session, topic: &str) -> Result<(), Error> {
    trace!("deleting topic {}", topic);
    //create topic map-message
    let mut msg: MapMessage = Default::default();
    msg.body
        .insert("dn".to_string(), TypedValue::String(topic.to_string()));
    msg.body.insert(
        "dt".to_string(),
        TypedValue::Integer(DESTINATION_TYPE_TOPIC),
    );
    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert("JMS_TIBCO_MSG_EXT".to_string(), TypedValue::Boolean(true));
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::DeleteDestination as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let result = session.send_message(&admin_queue, msg);
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("error while deleting topic {}: {}", topic, err);
            return Err(err);
        }
    }
    Ok(())
}

//
// Bridges
//

/// create a bridge
pub fn create_bridge(session: &Session, bridge: &BridgeInfo) -> Result<(), Error> {
    //create bridge map-message
    let mut msg: MapMessage = Default::default();
    match bridge.source.clone() {
        Destination::Queue(name) => {
            msg.body.insert(
                "st".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_QUEUE),
            );
            msg.body.insert("sn".to_string(), TypedValue::String(name));
        }
        Destination::Topic(name) => {
            msg.body.insert(
                "st".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_TOPIC),
            );
            msg.body.insert("sn".to_string(), TypedValue::String(name));
        }
    }

    match bridge.target.clone() {
        Destination::Queue(name) => {
            msg.body.insert(
                "tt".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_QUEUE),
            );
            msg.body.insert("tn".to_string(), TypedValue::String(name));
        }
        Destination::Topic(name) => {
            msg.body.insert(
                "tt".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_TOPIC),
            );
            msg.body.insert("tn".to_string(), TypedValue::String(name));
        }
    }
    if let Some(sel) = bridge.selector.clone() {
        msg.body.insert("sel".to_string(), TypedValue::String(sel));
    }
    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert("JMS_TIBCO_MSG_EXT".to_string(), TypedValue::Boolean(true));
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::CreateBridge as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let result = session.send_message(&admin_queue, msg);
    match result {
        Ok(_) => {}
        Err(err) => {
            error!(
                "error while creating bridge {:?}->{:?}: {}",
                bridge.source, bridge.target, err
            );
            return Err(err);
        }
    }
    Ok(())
}

/// delete a bridge
pub fn delete_bridge(session: &Session, bridge: &BridgeInfo) -> Result<(), Error> {
    //create bridge map-message
    let mut msg: MapMessage = Default::default();
    match bridge.source.clone() {
        Destination::Queue(name) => {
            msg.body.insert(
                "st".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_QUEUE),
            );
            msg.body.insert("sn".to_string(), TypedValue::String(name));
        }
        Destination::Topic(name) => {
            msg.body.insert(
                "st".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_TOPIC),
            );
            msg.body.insert("sn".to_string(), TypedValue::String(name));
        }
    }

    match bridge.target.clone() {
        Destination::Queue(name) => {
            msg.body.insert(
                "tt".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_QUEUE),
            );
            msg.body.insert("tn".to_string(), TypedValue::String(name));
        }
        Destination::Topic(name) => {
            msg.body.insert(
                "tt".to_string(),
                TypedValue::Integer(DESTINATION_TYPE_TOPIC),
            );
            msg.body.insert("tn".to_string(), TypedValue::String(name));
        }
    }
    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert("JMS_TIBCO_MSG_EXT".to_string(), TypedValue::Boolean(true));
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::DeleteBridge as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let result = session.send_message(&admin_queue, msg);
    match result {
        Ok(_) => {}
        Err(err) => {
            error!(
                "error while deleting bridge {:?}->{:?}: {}",
                bridge.source, bridge.target, err
            );
            return Err(err);
        }
    }
    Ok(())
}

//
// Server
//

/// get server state
///
/// the underlying connection must be an admin connection created through the tibco_ems::admin::connect() function.
pub fn get_server_state(session: &Session) -> Result<ServerState, Error> {
    const TIMEOUT: i64 = 60000;
    let mut msg: MapMessage = Default::default();

    //header
    let mut header: HashMap<String, TypedValue> = HashMap::new();
    //actual boolean
    header.insert(
        "code".to_string(),
        TypedValue::Integer(AdminCommands::GetStateInfo as i32),
    );
    header.insert("save".to_string(), TypedValue::Boolean(true));
    header.insert("arseq".to_string(), TypedValue::Integer(1));
    msg.header = Some(header);

    let admin_queue = Destination::Queue(ADMIN_QUEUE_NAME.to_string());
    let query_result = session.request_reply(&admin_queue, msg, TIMEOUT);
    match query_result {
        Ok(response) => {
            if let Some(resp) = response {
                match &resp {
                    Message::MapMessage(map_message) => {
                        //got response message
                        let state_str = extract!(
                            TypedValue::String(_),
                            map_message.body.get("state").unwrap()
                        )
                        .expect("extract server status");
                        if state_str == "3" {
                            return Ok(ServerState::Standby);
                        } else {
                            return Ok(ServerState::Active);
                        }
                    }
                    _ => {
                        warn!("unkown response from queue information request")
                    }
                }
            }
        }
        Err(err) => {
            error!(
                "something went wronge retrieving queue information: {}",
                err
            );
            return Err(err);
        }
    }
    Ok(ServerState::Active)
}

/// holds static bridge information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeInfo {
    /// source of the bridge
    pub source: Destination,
    /// target of the bridge
    pub target: Destination,
    /// selector
    pub selector: Option<String>,
}

/// available overflow policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverflowPolicy {
    /// default overflow policy
    Default = 0,
    /// discard old message if destination overflows
    DiscardOld = 1,
    /// reject incoming message if destination overflows
    RejectIncoming = 2,
}

/// admin command codes used on the admin queue
#[derive(Debug, Clone)]
pub enum AdminCommands {
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
    DeleteBridge = 221,
}

/// server states
#[derive(Debug, Clone)]
pub enum ServerState {
    /// server is standby
    Standby = 3,
    /// server is active
    Active = 4,
}
