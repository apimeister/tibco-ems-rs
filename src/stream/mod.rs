//! Tibco EMS streaming functions

use super::{BytesMessage, Connection, Consumer, Message, Session, TextMessage};
use futures::task::{Context, Poll};
use futures::Stream;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;

/// Represents a Message as a Stream
pub struct MessageStream<T> {
    /// Reference counting pointer to a Connection
    pub connection: Rc<Connection>,
    /// Reference counting pointer to a Session
    pub session: Rc<Session>,
    /// Reference counting pointer to a Consumer
    pub consumer: Rc<Consumer>,
    /// Optional Message
    pub message: Option<T>,
}

impl Stream for MessageStream<TextMessage> {
    type Item = TextMessage;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let consumer: Consumer = *self.consumer.deref();
        let result = consumer.receive_message(None);
        match result {
            Ok(result) => match result {
                Some(Message::TextMessage(ref text_message)) => {
                    Poll::Ready(Some(text_message.clone()))
                }
                _ => Poll::Ready(None),
            },
            Err(_err) => Poll::Ready(None),
        }
    }
}

impl Stream for MessageStream<BytesMessage> {
    type Item = BytesMessage;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let consumer: Consumer = *self.consumer.deref();
        let result = consumer.receive_message(None);
        match result {
            Ok(result) => match result {
                Some(Message::BytesMessage(ref bytes_message)) => {
                    Poll::Ready(Some(bytes_message.clone()))
                }
                _ => Poll::Ready(None),
            },
            Err(_err) => Poll::Ready(None),
        }
    }
}
