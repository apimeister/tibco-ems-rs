#[cfg(test)]
mod admin {

    use tibco_ems::{
        admin::{BridgeInfo, OverflowPolicy, QueueInfo, TopicInfo},
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
}
