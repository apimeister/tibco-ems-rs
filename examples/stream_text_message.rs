use tibco_ems::Destination;
use tibco_ems::TextMessage;
use futures::StreamExt;

#[tokio::main]
async fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  let destination = Destination::Queue("myqueue".to_string());
  let mut stream = connection.open_stream::<TextMessage>(&destination, None).unwrap();
  while let Some(msg) = stream.next().await {
    println!("msg: {}",msg.body);
  }
}
