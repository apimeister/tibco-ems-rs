# 0.3.5 2021-06-14

* fixed: destination header returned wrong value

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

