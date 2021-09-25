use tibco_ems::admin::BridgeInfo;
use tibco_ems::Destination;

fn main() {
  let url = "tcp://localhost:7222";
  let user = "admin";
  let password = "admin";

  let connection = tibco_ems::admin::connect(url, user, password).unwrap();
  let session = connection.session().unwrap();

  let bridge = BridgeInfo {
    source: Destination::Topic("test.t".to_string()),
    target: Destination::Queue("test.q".to_string()),
    selector: None,
  };
  let result = tibco_ems::admin::delete_bridge(&session, &bridge);
  println!("{:?}", result);
}
