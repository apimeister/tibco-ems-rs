# tibco-ems-rs

[![Latest Version](https://img.shields.io/crates/v/tibco_ems.svg)](https://crates.io/crates/tibco_ems)
[![codecov](https://codecov.io/gh/apimeister/tibco-ems-rs/branch/main/graph/badge.svg?token=NVYPCNKU0M)](https://codecov.io/gh/apimeister/tibco-ems-rs)  
A high-level API for the Tibco EMS C library

## Build

To build this crate, the TIBCO EMS C library must either be in the LD_LIBRARY_PATH or alternatively a EMS_HOME environment variable must be set.

Please check the [CHANGELOG](./CHANGELOG.md) when upgrading.

## Examples

Sending a text message to a queue.

```rust
use tibco_ems::Destination;
use tibco_ems::TextMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  let session = connection.session().unwrap();

  let msg = TextMessage{
    body:"hallo welt".to_string(),
    ..Default::default()
  };

  let destination = Destination::Queue("myqueue".to_string());
  
  let _ignore = session.send_message(&destination, msg);
}
```

Receiving a text message from a queue.

```rust
use tibco_ems::Destination;
use tibco_ems::Message;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url,user,password).unwrap();
  let session = connection.session().unwrap();

  let destination = Destination::Queue("myqueue".to_string());
  let consumer = session.queue_consumer(&destination, None).unwrap();
  
  println!("waiting 10 seconds for a message");
  let msg_result = consumer.receive_message(Some(10000));

  match msg_result {
    Ok(result_value) => {
      match result_value {
        Some(message) => {
          match &message {
            Message::TextMessage(text_message) =>{
              println!("received text message");
              println!("content: {}", text_message.body);
            },
            _ => {
              println!("unknown type");
            }
          }    
        },
        None =>{
          println!("no message returned");
        },
      }
    },
    Err(status) => {
      println!("returned status: {:?}",status);
    }
  }
}
```

More examples can be found in the examples directory.

## License

tibco-ems-rs is licensed under [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) [LICENSE](LICENSE).  
TIBCO Enterprise Messaging Service, and all related components therein are property of TIBCO Software, and are not provided with this software package. Refer to your own TIBCO License terms for details.
