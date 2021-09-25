use tibco_ems::admin::BridgeInfo;
use tibco_ems::admin::QueueInfo;
use tibco_ems::admin::TopicInfo;
use tibco_ems::Destination;

fn main() {
  let url = "tcp://localhost:7222";
  let user = "admin";
  let password = "admin";

  let connection = tibco_ems::admin::connect(url, user, password).unwrap();
  let session = connection.session().unwrap();

  //create source topic
  let topic = TopicInfo {
    name: "test.t".to_string(),
    ..Default::default()
  };
  let _result = tibco_ems::admin::create_topic(&session, &topic);
  //create target queue
  let queue = QueueInfo {
    name: "test.q".to_string(),
    ..Default::default()
  };
  let _result = tibco_ems::admin::create_queue(&session, &queue);
  //create bridge
  let bridge = BridgeInfo {
    source: Destination::Topic("test.t".to_string()),
    target: Destination::Queue("test.q".to_string()),
    selector: None,
  };
  let result = tibco_ems::admin::create_bridge(&session, &bridge);
  println!("{:?}", result);
}
