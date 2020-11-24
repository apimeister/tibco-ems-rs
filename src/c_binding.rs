use std::ffi::c_void;
use std::os::raw::c_char;

#[allow(dead_code)]
extern "C" {
  /// Create a new error context object.
  pub fn tibemsErrorContext_Create(
    errorContext: *mut c_void) -> tibems_status;
  /// Retrieve any available detailed error string associated with the last EMS call.
  pub fn tibemsErrorContext_GetLastErrorString(
    errorContext: *mut c_void,
    str: *mut c_char) -> tibems_status;
  /// Create a new SSL parameter object.
  pub fn tibemsSSLParams_Create() -> *mut c_void;
  /// Destroy an SSL parameter object.
  pub fn tibemsSSLParams_Destroy(sslParams: *mut c_void);
  /// Get the text string corresponding to a status code.
  pub fn tibemsStatus_GetText(status: tibems_status) -> *const c_char;
  /// Create an administration connection to a server.
  pub fn tibemsAdmin_Create(admin: *mut tibemsAdmin,
    url: *const c_char, userName: *const c_char,
    password: *const c_char, sslparams: *mut c_void) -> tibems_status;
  /// Close the administrative connection to the server.
  pub fn tibemsAdmin_Close(admin: tibemsAdmin) -> tibems_status;
  /// Get the current set of server metrics.
  pub fn tibemsAdmin_GetInfo(
    admin: tibemsAdmin,
    serverInfo: *mut usize) -> tibems_status;
  /// Get information about a destination of the given name.
  pub fn tibemsAdmin_GetDestination(
    admin: tibemsAdmin,
    destInfo: *mut usize,
    destName: *const c_char,
    destType: usize) -> tibems_status;
  /// Get the destinations that match the given pattern and the given permanence type.
  pub fn tibemsAdmin_GetDestinations(
    admin: tibemsAdmin,
    collection: *mut usize,
    pattern: *const c_char,
    destType: tibemsDestinationType,
    permType: tibems_permType,
    statOnly: tibems_bool) ->tibems_status;
  /// Get the command timeout.
  pub fn tibemsAdmin_GetCommandTimeout(
    admin: tibemsAdmin,
    timeout: *mut i64);
  /// Set the command timeout.
  pub fn tibemsAdmin_SetCommandTimeout(
    admin: tibemsAdmin,
    timeout: i64) -> tibems_status;
  /// Get the total number of queues in the server.
  pub fn tibemsServerInfo_GetQueueCount(
    serverInfo: usize,
    count: *mut usize) -> tibems_status;
  /// Create a tibemsDestinationInfo object.
  pub fn tibemsDestinationInfo_Create(
    destInfo: *mut usize,
    destName: *const c_char,
    destType: usize) -> tibems_status;
  /// Destroy a tibemsDestinationInfo object.
  pub fn tibemsDestinationInfo_Destroy(
    destInfo: usize) -> tibems_status;
  /// Get the total number of pending messages for this destination.
  pub fn tibemsDestinationInfo_GetPendingMessageCount(
    destInfo: usize,
    count: *mut i64 ) -> tibems_status;
  /// Get the number of active consumers on this destination.
  pub fn tibemsDestinationInfo_GetConsumerCount(
    destInfo: usize,
    count: &mut usize) -> tibems_status;
  /// Get the overflow policy for this destination.
  pub fn tibemsDestinationInfo_GetOverflowPolicy(
    destInfo: usize,
    overflowPolicy: *mut usize) -> tibems_status;
  /// Get the name of this destination.
  pub fn tibemsDestinationInfo_GetName(
    destInfo: usize,
    name: *const c_char,
    name_len: usize) -> tibems_status;
  /// Get the first object in a collection.
  pub fn tibemsCollection_GetFirst(
    collection: usize,
    collection_ptr: *mut usize) -> tibems_status;
  /// Get the next object in a collection.
  pub fn tibemsCollection_GetNext(
    collection: usize,
    collection_ptr: *mut usize) -> tibems_status;
  /// Destroy a collection.
  pub fn tibemsCollection_Destroy(
    collection: usize) -> tibems_status;
  /// Get the current number of subscriptions for this topic.
  pub fn tibemsTopicInfo_GetSubscriptionCount(
    topicInfo: usize,
    count: *mut i64) -> tibems_status;
  /// Get the current number of durable subscriptions for this topic.
  pub fn tibemsTopicInfo_GetDurableSubscriptionCount(
    topicInfo: usize,
    count: *mut i64) -> tibems_status;
  /// Create a connection factory.
  pub fn tibemsConnectionFactory_Create() -> *mut tibemsConnectionFactory;
  /// Destroy a connection factory object.
  pub fn tibemsConnectionFactory_Destroy(factory: *mut tibemsConnectionFactory) -> tibems_status;
  /// Set the server URL.
  pub fn tibemsConnectionFactory_SetServerURL(
    factory: *mut tibemsConnectionFactory,
    url: *const c_char) -> tibems_status;
  /// Set a connection factory’s username.
  pub fn tibemsConnectionFactory_SetUserName(
    factory: *mut tibemsConnectionFactory,
    username: *const c_char) -> tibems_status;
  /// Set the password used by the connection factory to authenticate itself with the EMS Server.
  pub fn tibemsConnectionFactory_SetUserPassword(
    factory: *mut tibemsConnectionFactory,
    password: *const c_char) -> tibems_status;
  /// Create a connection object.
  pub fn tibemsConnectionFactory_CreateConnection(
    factory: *mut tibemsConnectionFactory,
    connection: *mut usize,
    username: *const c_char,
    password: *const c_char) -> tibems_status;
  /// Start delivering inbound messages
  pub fn tibemsConnection_Start(
    connection: usize) -> tibems_status;
  /// Create a session object.
  pub fn tibemsConnection_CreateSession(
    connection: usize,
    session: *mut usize,
    transacted: tibems_bool,
    acknowledgeMode: tibemsAcknowledgeMode) -> tibems_status;
  /// Close a session; reclaim resources.
  pub fn tibemsSession_Close(session: usize) -> tibems_status;
  /// Create a destination object.
  pub fn tibemsDestination_Create(
    destination: *mut usize,
    destType: tibemsDestinationType,
    name: *const c_char) -> tibems_status;
  /// Destroy a destination object.
  pub fn tibemsDestination_Destroy(
    destination: usize) -> tibems_status;
  /// Create a message producer.
  pub fn tibemsSession_CreateProducer(
    session: usize,
    producer: *mut usize,
    destination: usize ) -> tibems_status;
  /// Create a text message.
  pub fn tibemsSession_CreateTextMessage(
    session: usize,
    textMsg: *mut tibemsMsg) -> tibems_status;
  /// Create a message.
  pub fn tibemsSession_CreateMessage(
    session: usize,
    textMsg: *mut usize) -> tibems_status;
  /// Send a message.
  pub fn tibemsMsgProducer_Send(
    msgProducer: usize,
    message: usize) -> tibems_status;
  /// Destroy the producer object; reclaim resources.
  pub fn tibemsMsgProducer_Close(
    msgProducer: usize) -> tibems_status;
  pub fn tibemsQueue_Create(
    queue: *mut tibemsDestination,
    queueName: *const c_char) -> tibems_status;
  /// Create a new EMS lookup context object.
  pub fn tibemsLookupContext_Create(
    context: *mut tibemsLookupContext,
    brokerURL: *const c_char,
    username: *const c_char,
    password: *const c_char) -> tibems_status;
  /// Look up an object in the naming server.
  pub fn tibemsLookupContext_LookupDestination(
    context: tibemsLookupContext,
    name: *const c_char,
    destination: *mut tibemsDestination) -> tibems_status;
  /// Set the data string of a text message.
  pub fn tibemsTextMsg_SetText(
    message: usize,
    text: *const c_char) -> tibems_status;
  /// Destroy a message.
  pub fn tibemsMsg_Destroy(
    message: usize) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetStringProperty(
    message: usize, 
    name: *const c_char,
    value: *const c_char) -> tibems_status;
  /// Create a text message.
  pub fn tibemsTextMsg_Create(
    message: *mut usize) -> tibems_status;
  /// Create a message consumer
  pub fn tibemsSession_CreateConsumer(
    session: usize,
    consumer: *mut usize,
    destination: usize,
    messageSelector: *const c_char,
    noLocal: tibems_bool) -> tibems_status;
  /// Receive a message (synchronous).
  pub fn tibemsMsgConsumer_Receive(
    msgConsumer: usize,
    message: *mut usize) -> tibems_status;
  /// Receive a message (synchronous, blocks up to a time limit).
  pub fn tibemsMsgConsumer_ReceiveTimeout(
    msgConsumer: usize,
    message: *mut usize,
    timeout: i64) -> tibems_status;
  /// Get the body type of a message.
  pub fn tibemsMsg_GetBodyType(
    message: usize,
    bodyType: *mut tibemsMsgType) -> tibems_status;
  /// Get the string data from a text message.
  pub fn tibemsTextMsg_GetText(
    message: usize,
    text: *const *const c_char) -> tibems_status;
}

#[allow(dead_code)]
#[repr(C)]
pub struct tibemsErrorContext { pub _val: [u8; 0] }

#[allow(dead_code)]
#[repr(C)]
pub struct tibemsConnectionFactory { pub _val: [u8; 0] }

#[allow(dead_code)]
#[repr(C)]
pub struct tibemsDestination { pub _val: usize }

#[allow(dead_code)]
#[repr(C)]
pub struct tibemsMsg { pub _val: usize }

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct tibemsAdmin { pub _val: usize }

#[allow(dead_code)]
#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct tibemsLookupContext { pub _val: [u8; 0] }

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsMsgType{
  TIBEMS_MESSAGE_UNKNOWN                      = 0,
  TIBEMS_MESSAGE                              = 1,
  TIBEMS_BYTES_MESSAGE                        = 2,
  TIBEMS_MAP_MESSAGE                          = 3,
  TIBEMS_OBJECT_MESSAGE                       = 4,
  TIBEMS_STREAM_MESSAGE                       = 5,
  TIBEMS_TEXT_MESSAGE                         = 6,
  TIBEMS_MESSAGE_UNDEFINED                    = 256
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsDestinationType{
  TIBEMS_UNKNOWN                              = 0,
  TIBEMS_QUEUE                                = 1,
  TIBEMS_TOPIC                                = 2,
  TIBEMS_DEST_UNDEFINED                       = 256
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibems_bool{
  TIBEMS_FALSE  = 0,
  TIBEMS_TRUE   = 1
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibems_permType{
  TIBEMS_DEST_GET_STATIC = 1,
  TIBEMS_DEST_GET_DYNAMIC = 2,
  TIBEMS_DEST_GET_NOTEMP = 3,
  TIBEMS_DEST_GET_ALL = 4,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsAcknowledgeMode{
  TIBEMS_SESSION_TRANSACTED                   = 0,
  TIBEMS_AUTO_ACKNOWLEDGE                     = 1,
  TIBEMS_CLIENT_ACKNOWLEDGE                   = 2,
  TIBEMS_DUPS_OK_ACKNOWLEDGE                  = 3,
  /// Extensions to the JMS spec
  TIBEMS_NO_ACKNOWLEDGE                       = 22,
  /// Extensions to the JMS spec
  TIBEMS_EXPLICIT_CLIENT_ACKNOWLEDGE          = 23,
  /// Extensions to the JMS spec
  TIBEMS_EXPLICIT_CLIENT_DUPS_OK_ACKNOWLEDGE  = 24
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum tibems_status{
  /// The call completed normally.
  TIBEMS_OK                          = 0,
  /// A function call or server request occurred in an inappropriate context. For example, tibemsSession_Commit indicates this status when the session is non-transactional.
  TIBEMS_ILLEGAL_STATE               = 1,
  /// The provider rejects the connection’s client ID. Setting a connection’s client ID to an invalid or duplicate value results in this exception. (A duplicate value is one that is already in use by another connection.)
  TIBEMS_INVALID_CLIENT_ID           = 2,
  /// tibemsd cannot locate the destination.
  TIBEMS_INVALID_DESTINATION         = 3,
    TIBEMS_INVALID_SELECTOR            = 4,
  ///Non-specific error code.
  TIBEMS_EXCEPTION                   = 5,
    TIBEMS_SECURITY_EXCEPTION          = 6,

    TIBEMS_MSG_EOF                     = 7,

    TIBEMS_MSG_NOT_READABLE            = 9,
    TIBEMS_MSG_NOT_WRITEABLE           = 10,

    TIBEMS_SERVER_NOT_CONNECTED        = 11,
    TIBEMS_VERSION_MISMATCH            = 12,
    TIBEMS_SUBJECT_COLLISION           = 13,

    TIBEMS_INVALID_PROTOCOL            = 15,
    TIBEMS_INVALID_HOSTNAME            = 17,
    TIBEMS_INVALID_PORT                = 18,
    TIBEMS_NO_MEMORY                   = 19,
  /// The function received an illegal value as an argument.
  TIBEMS_INVALID_ARG                 = 20,
  /// The server has exceeded the maximum number of licensed connections or hosts that it can service.
  TIBEMS_SERVER_LIMIT                = 21,

    TIBEMS_MSG_DUPLICATE               = 22,

    TIBEMS_SERVER_DISCONNECTED         = 23,
    TIBEMS_SERVER_RECONNECTING         = 24,

    TIBEMS_NOT_PERMITTED               = 27,

    TIBEMS_SERVER_RECONNECTED          = 28,

    TIBEMS_INVALID_NAME                = 30,
    TIBEMS_INVALID_TYPE                = 31,
    TIBEMS_INVALID_SIZE                = 32,
    TIBEMS_INVALID_COUNT               = 33,
    TIBEMS_NOT_FOUND                   = 35,
    TIBEMS_ID_IN_USE                   = 36,
    TIBEMS_ID_CONFLICT                 = 37,
    TIBEMS_CONVERSION_FAILED           = 38,

    TIBEMS_INVALID_MSG                 = 42,
    TIBEMS_INVALID_FIELD               = 43,
    TIBEMS_INVALID_INSTANCE            = 44,
    TIBEMS_CORRUPT_MSG                 = 45,

    TIBEMS_PRODUCER_FAILED             = 47,
  ///The timeout has expired while waiting for a message. See tibemsMsgConsumer_ReceiveTimeout.
  TIBEMS_TIMEOUT                     = 50,
    TIBEMS_INTR                        = 51,
    TIBEMS_DESTINATION_LIMIT_EXCEEDED  = 52,
    TIBEMS_MEM_LIMIT_EXCEEDED          = 53,
    TIBEMS_USER_INTR                   = 54,

    TIBEMS_INVALID_QUEUE_GROUP         = 63,
    TIBEMS_INVALID_TIME_INTERVAL       = 64,
    TIBEMS_INVALID_IO_SOURCE           = 65,
    TIBEMS_INVALID_IO_CONDITION        = 66,
    TIBEMS_SOCKET_LIMIT                = 67,

    TIBEMS_OS_ERROR                    = 68,

    TIBEMS_WOULD_BLOCK                 = 69,

    TIBEMS_INSUFFICIENT_BUFFER         = 70,

    TIBEMS_EOF                         = 71,
    TIBEMS_INVALID_FILE                = 72,
    TIBEMS_FILE_NOT_FOUND              = 73,
  /// An operating system I/O call failed.
  TIBEMS_IO_FAILED                   = 74,

    TIBEMS_NOT_FILE_OWNER              = 80,

    TIBEMS_ALREADY_EXISTS              = 91,

    TIBEMS_INVALID_CONNECTION          = 100,
    TIBEMS_INVALID_SESSION             = 101,
    TIBEMS_INVALID_CONSUMER            = 102,
    TIBEMS_INVALID_PRODUCER            = 103,
    TIBEMS_INVALID_USER                = 104,
    TIBEMS_INVALID_GROUP               = 105,

    TIBEMS_TRANSACTION_FAILED          = 106,
    TIBEMS_TRANSACTION_ROLLBACK        = 107,
    TIBEMS_TRANSACTION_RETRY           = 108,

    TIBEMS_INVALID_XARESOURCE          = 109,

    TIBEMS_FT_SERVER_LACKS_TRANSACTION = 110,

    TIBEMS_LDAP_ERROR                  = 120,
    TIBEMS_INVALID_PROXY_USER          = 121,

  /// SSL detected an invalid X.509 certificate.
  TIBEMS_INVALID_CERT                = 150,
  /// SSL detected an X.509 certificate that is not yet valid; that is, the current date is before the first date for which the certificate becomes valid.
  TIBEMS_INVALID_CERT_NOT_YET        = 151,   
    TIBEMS_INVALID_CERT_EXPIRED        = 152,
    TIBEMS_INVALID_CERT_DATA           = 153,
    TIBEMS_ALGORITHM_ERROR             = 154,
    TIBEMS_SSL_ERROR                   = 155,   
    TIBEMS_INVALID_PRIVATE_KEY         = 156,
    TIBEMS_INVALID_ENCODING            = 157,
    TIBEMS_NOT_ENOUGH_RANDOM           = 158,
    TIBEMS_INVALID_CRL_DATA            = 159,
    TIBEMS_CRL_OFF                     = 160,
    TIBEMS_EMPTY_CRL                   = 161,

    TIBEMS_NOT_INITIALIZED             = 200,
    TIBEMS_INIT_FAILURE                = 201,
    TIBEMS_ARG_CONFLICT                = 202,
    TIBEMS_SERVICE_NOT_FOUND           = 210,
    TIBEMS_INVALID_CALLBACK            = 211,
    TIBEMS_INVALID_QUEUE               = 212,
    TIBEMS_INVALID_EVENT               = 213,
    TIBEMS_INVALID_SUBJECT             = 214,
    TIBEMS_INVALID_DISPATCHER          = 215,
    
    /* JVM related errors */
    TIBEMS_JNI_EXCEPTION               = 230,
    TIBEMS_JNI_ERR                     = 231,
    TIBEMS_JNI_EDETACHED               = 232,
    TIBEMS_JNI_EVERSION                = 233,
    TIBEMS_JNI_EEXIST                  = 235,
    TIBEMS_JNI_EINVAL                  = 236,

    TIBEMS_NO_MEMORY_FOR_OBJECT        = 237,

    TIBEMS_UFO_CONNECTION_FAILURE      = 240,
  /// The function is not implemented.
  TIBEMS_NOT_IMPLEMENTED             = 255
}