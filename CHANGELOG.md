# 0.4.9 2022-04-27

* fix broken null check for correlationid

# 0.4.8 2022-04-26

* fix reading binary messages with null character inside

# 0.4.7 2022-03-22

* do not SetBytes in case of the empty payload

# 0.4.6 2022-03-17

* check null pointer in correlationid
* support `setCorrelationId` theough the "CorrelationID" header

# 0.4.5 2022-03-17

* fix zero byte message parsing

# 0.4.4 2022-02-09

* fix null pointer in correlationid extraction

# 0.4.3 2022-02-08

* extract correlation id from message header
* apply clippy recommendations

# 0.4.2 2021-12-08

* truncate message body buffer for binary messages

# 0.4.1 2021-10-05

* fix typo in admin commands enum

# 0.4.0 2021-09-25

* support object message
* apply cargo format
* initial mock server support

# 0.3.12 2021-09-17

* fix binary message content extraction

# 0.3.11 2021-09-11

* fix map message in map message unpacking

# 0.3.10 2021-09-06

* implement clippy recommendations
* support prefetch property

# 0.3.9 2021-08-13

* ignore binary content in map messages, so no error pops up

# 0.3.8 2021-07-27

* make `Message` serializable/deserializable

# 0.3.7 2021-07-01

* refine error returns for admin functions

# 0.3.6 2021-06-14

* fixed: destination header returned wrong value
* add MessageID to header

# 0.3.4 2021-06-14

* add type object_message (handled an binary message internally)
* add destination to message header
* reformat readme sample code

# 0.3.3 2021-06-09

* topic subcription support

# 0.3.2 2021-06-05

* make connection handle atomic (type changed from RC<usize> to Arc<usize>)

# 0.3.1 2021-05-26

* fix samples in the readme

# 0.3.0 2021-05-25

* refactor Destination to enum
* provide stream trait for message consumer
* borrow Destination on send message
* refactor TypedValue to enum

# 0.2.17 2021-05-17

* refine error return on send_message and request_reply

