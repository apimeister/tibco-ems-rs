use tibco_ems::admin::QueueInfo;

fn main() {
  let url = "tcp://localhost:7222";
  let user = "admin";
  let password = "admin";

  let connection = tibco_ems::admin::connect(url, user, password).unwrap();
  let session = connection.session().unwrap();

  let queue = QueueInfo {
    name: "test.q".to_string(),
    ..Default::default()
  };
  let result = tibco_ems::admin::create_queue(&session, &queue);
  println!("{:?}", result);
}
