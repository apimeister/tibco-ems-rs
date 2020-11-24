# tibco_ems
[![Latest Version](https://img.shields.io/crates/v/tibco_ems.svg)](https://crates.io/crates/tibco_ems)

Rust bindings for the Tibco EMS C library.

A high-level API is provided to simplify the interaction. 
Since the high-level API does not fully cover the EMS capabilities, a low-level binding in provided by the sub-module tibco_ems::c_binding.


# License
tibco_ems is licensed under Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0).

TIBCO Enterprise Messaging Service, and all related components therein are property of TIBCO Software, and are not provided with this software package. Refer to your own TIBCO License terms for details.

# Build

To build this crate, the TIBCO EMS C library must either be in the LD_LIBRARY_PATH or alternatively a EMS_HOME environment variable must be set.

## Usage

Put this in your `Cargo.toml`:

```text
[dependencies]
tibco_ems = "0.1"
```

## Examples

Sending a text message to a queue.

```rust
use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();
  {
    let session = connection.session().unwrap();

    let msg = TextMessage{body:"hallo welt".to_string(),header: None};

    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let _ignore = session.send_message(destination, msg.into());
  }
}
```

Receiving a text message from a queue.

```rust
use tibco_ems::Destination;
use tibco_ems::DestinationType;
use tibco_ems::TextMessage;
use tibco_ems::MessageType;

fn main() {
  let url = "tcp://localhost:7222";
  let user="admin";
  let password="admin";

  let connection = tibco_ems::connect(url.to_string(),user.to_string(),password.to_string()).unwrap();
  {
    let session = connection.session().unwrap();

    let destination = Destination{
      destination_type: DestinationType::Queue,
      destination_name: "myqueue".to_string(),
    };
    let consumer = session.queue_consumer(destination,None).unwrap();
    
    println!("waiting 10 seconds for a message");
    let msg_result = tibco_ems::receive_message(consumer, Some(10000));

    match msg_result {
      Ok(result_value) => {
        match result_value {
          Some(message) => {
            match message.message_type {
              MessageType::TextMessage =>{
                println!("received text message");
                let text_message = TextMessage::from(message);
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
}
```

More examples can be found in the examples directory.
