#[cfg(test)]
mod admin {

    use tibco_ems::{
        admin::{AdminCommands, BridgeInfo, OverflowPolicy, QueueInfo, ServerState, TopicInfo},
        Destination,
    };

    #[test]
    fn test_queue_info_default() {
        let queue_info = QueueInfo::default();

        // Ensure that the default values are set correctly
        assert_eq!(queue_info.name, String::new());
        assert_eq!(queue_info.pending_messages, None);
        assert_eq!(queue_info.max_messages, None);
        assert_eq!(queue_info.max_bytes, None);
        assert_eq!(queue_info.overflow_policy, None);
        assert_eq!(queue_info.failsafe, None);
        assert_eq!(queue_info.secure, None);
        assert_eq!(queue_info.global, None);
        assert_eq!(queue_info.sender_name, None);
        assert_eq!(queue_info.sender_name_enforced, None);
        assert_eq!(queue_info.prefetch, None);
        assert_eq!(queue_info.expiry_override, None);
        assert_eq!(queue_info.redelivery_delay, None);
        assert_eq!(queue_info.consumer_count, None);
        assert_eq!(queue_info.incoming_total_count, None);
        assert_eq!(queue_info.outgoing_total_count, None);
    }

    #[test]
    fn test_topic_info_default() {
        let topic_info = TopicInfo::default();

        // Ensure that the default values are set correctly
        assert_eq!(topic_info.name, String::new());
        assert_eq!(topic_info.expiry_override, None);
        assert_eq!(topic_info.global, None);
        assert_eq!(topic_info.max_bytes, None);
        assert_eq!(topic_info.max_messages, None);
        assert_eq!(topic_info.overflow_policy, None);
        assert_eq!(topic_info.prefetch, None);
        assert_eq!(topic_info.durable_count, None);
        assert_eq!(topic_info.subscriber_count, None);
        assert_eq!(topic_info.pending_messages, None);
        assert_eq!(topic_info.incoming_total_count, None);
        assert_eq!(topic_info.outgoing_total_count, None);
    }

    #[test]
    fn test_bridge_info_clone() {
        let bridge_info = BridgeInfo {
            source: Destination::Queue("source_queue".to_string()),
            target: Destination::Topic("target_topic".to_string()),
            selector: Some("some_selector".to_string()),
        };

        // Clone the BridgeInfo
        let cloned_bridge_info = bridge_info.clone();

        // Ensure that the cloned BridgeInfo is equal to the original
        assert_eq!(cloned_bridge_info, bridge_info);
    }

    #[test]
    fn test_overflow_policy_enum() {
        let default_policy: OverflowPolicy = OverflowPolicy::Default;
        let discard_old_policy: OverflowPolicy = OverflowPolicy::DiscardOld;
        let reject_incoming_policy: OverflowPolicy = OverflowPolicy::RejectIncoming;

        // Ensure that the enum variants have the correct values
        assert_eq!(default_policy as i32, 0);
        assert_eq!(discard_old_policy as i32, 1);
        assert_eq!(reject_incoming_policy as i32, 2);
    }

    #[test]
    fn test_admin_commands_delete_destination() {
        let command = AdminCommands::DeleteDestination;
        assert_eq!(command, AdminCommands::DeleteDestination);
        assert_eq!(command as u8, 16);
    }

    #[test]
    fn test_admin_commands_create_destination() {
        let command = AdminCommands::CreateDestination;
        assert_eq!(command, AdminCommands::CreateDestination);
        assert_eq!(command as u8, 18);
    }

    #[test]
    fn test_admin_commands_list_destination() {
        let command = AdminCommands::ListDestination;
        assert_eq!(command, AdminCommands::ListDestination);
        assert_eq!(command as u8, 19);
    }

    #[test]
    fn test_admin_commands_get_server_info() {
        let command = AdminCommands::GetServerInfo;
        assert_eq!(command, AdminCommands::GetServerInfo);
        assert_eq!(command as u8, 120);
    }

    #[test]
    fn test_admin_commands_get_state_info() {
        let command = AdminCommands::GetStateInfo;
        assert_eq!(command, AdminCommands::GetStateInfo);
        assert_eq!(command as u8, 127);
    }

    #[test]
    fn test_admin_commands_create_bridge() {
        let command = AdminCommands::CreateBridge;
        assert_eq!(command, AdminCommands::CreateBridge);
        assert_eq!(command as u8, 220);
    }

    #[test]
    fn test_admin_commands_delete_bridge() {
        let command = AdminCommands::DeleteBridge;
        assert_eq!(command, AdminCommands::DeleteBridge);
        assert_eq!(command as u8, 221);
    }

    #[test]
    fn test_server_state_standby() {
        let state = ServerState::Standby;
        assert_eq!(state, ServerState::Standby);
        assert_eq!(state as u8, 3);
    }

    #[test]
    fn test_server_state_active() {
        let state = ServerState::Active;
        assert_eq!(state, ServerState::Active);
        assert_eq!(state as u8, 4);
    }
}

#[cfg(feature = "integration-tests")]
#[cfg(test)]
mod admin_integration {
    use tibco_ems::admin::{list_all_queues, QueueInfo, list_all_topics, create_queue, delete_queue, create_topic, delete_topic, TopicInfo, create_bridge, delete_bridge, BridgeInfo, get_server_state};

    const USER: &str = "admin";
    const PASSWORD: &str = "";
    const URL: &str = "tcp://localhost:7222";

    #[test]
    fn test_admin_connection_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD);
        assert!(con.is_ok());
    }

    #[test]
    fn test_admin_connection_failure() {
        let con = tibco_ems::admin::connect(URL, USER, "PASSWORD");
        assert!(con.is_err());
    }

    #[test]
    fn test_admin_list_all_queues_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let queues_res = list_all_queues(&session);
        assert!(queues_res.is_ok());
        assert!(queues_res.unwrap().len() >= 1);
    }

    //FIXME: implement failures

    #[test]
    fn test_admin_list_all_topics_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let topics_res = list_all_topics(&session);
        assert!(topics_res.is_ok());
        assert!(topics_res.unwrap().len() >= 1);
    }

    #[test]
    fn test_admin_create_delete_queue_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let queue = QueueInfo { name: "create_queue".into(), ..Default::default()};
        let create_res = create_queue(&session, &queue);
        assert!(create_res.is_ok());
        let delete_res = delete_queue(&session, "create_queue");
        assert!(delete_res.is_ok());
    }

    #[test]
    fn test_admin_create_delete_topic_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let topic = TopicInfo { name: "create_topic".into(), ..Default::default()};
        let create_res = create_topic(&session, &topic);
        assert!(create_res.is_ok());
        let delete_res = delete_topic(&session, "create_topic");
        assert!(delete_res.is_ok());
    }

    #[test]
    fn test_admin_create_delete_bridge_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let queue = QueueInfo { name: "create_bridge".into(), ..Default::default()};
        let _ = create_queue(&session, &queue).unwrap();
        let topic = TopicInfo { name: "create_bridge".into(), ..Default::default()};
        let _ = create_topic(&session, &topic).unwrap();
        let queue = tibco_ems::Destination::Queue("create_bridge".into());
        let topic = tibco_ems::Destination::Topic("create_bridge".into());
        let bridge = BridgeInfo { source: topic, target: queue, selector: None};
        let create_res = create_bridge(&session, &bridge);
        assert!(create_res.is_ok());
        let delete_res = delete_bridge(&session, &bridge);
        assert!(delete_res.is_ok());
        let _ = delete_queue(&session, "create_bridge");
        let _ = delete_topic(&session, "create_bridge");
    }

    #[test]
    fn test_admin_get_server_state_success() {
        let con = tibco_ems::admin::connect(URL, USER, PASSWORD).unwrap();
        let session = con.session().unwrap();
        let server_state_res = get_server_state(&session);
        assert!(server_state_res.is_ok());
    }

}
