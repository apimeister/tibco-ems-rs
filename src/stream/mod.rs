use futures::Stream;
use super::Connection;
use super::Session;
use super::Consumer;
use super::TextMessage;
use super::BytesMessage;
use super::Message;
use std::pin::Pin;
use futures::task::Context;
use futures::task::Poll;
use std::rc::Rc;
use std::ops::Deref;

pub struct MessageStream<T>{
  pub connection: Rc<Connection>,
  pub session: Rc<Session>,
  pub consumer: Rc<Consumer>,
  pub message: Option<T>,
}

impl Stream for MessageStream<TextMessage> {
  type Item = TextMessage;
  
  fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>{
    let consumer: Consumer = *self.consumer.deref();
    let result = consumer.receive_message(None);
    match result {
      Ok(result) => {
        match result {
          Some(msg) => {
            match &msg {
              Message::TextMessage(text_message) => {
                Poll::Ready(Some(text_message.clone()))
              },
              _ => {
                Poll::Ready(None)
              }
            }
          },
          None => {
            Poll::Ready(None)
          }
        }
      },
      Err(_err) => {
        Poll::Ready(None)
      },
    }
  }
}


impl<'dest> Stream for MessageStream<BytesMessage> {
  type Item = BytesMessage;
  
  fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Option<Self::Item>>{
    let consumer: Consumer = *self.consumer.deref();
    let result = consumer.receive_message(None);
    match result {
      Ok(result) => {
        match result {
          Some(msg) => {
            match &msg {
              Message::BytesMessage(bytes_message) => {
                Poll::Ready(Some(bytes_message.clone()))
              },
              _ => {
                Poll::Ready(None)
              }
            }
          },
          None => {
            Poll::Ready(None)
          }
        }
      },
      Err(_err) => {
        Poll::Ready(None)
      },
    }
  }
}
