use tibco_ems::admin::TopicInfo;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::admin::connect(url,user,password).unwrap();
  let session = connection.session().unwrap();

  let topic = TopicInfo{
    name: "test.t".to_string(),
    ..Default::default()
  };
  let result = tibco_ems::admin::create_topic(&session,&topic);
  println!("{:?}",result);
}
