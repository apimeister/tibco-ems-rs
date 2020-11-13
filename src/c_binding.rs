use std::ffi::c_void;
use std::os::raw::c_char;

#[allow(dead_code)]
extern "C" {
  pub fn tibemsErrorContext_Create(
      errorContext: *mut c_void) -> tibems_status;
  pub fn tibemsErrorContext_GetLastErrorString(
      errorContext: *mut c_void,
      str: *mut c_char) -> tibems_status;
  pub fn tibemsSSLParams_Create() -> *mut c_void;
  pub fn tibemsSSLParams_Destroy(sslParams: *mut c_void);
  pub fn tibemsStatus_GetText(status: tibems_status) -> *const c_char;
  pub fn tibemsDestination_Create(destination: *mut usize,
      destType: tibemsDestinationType,
      name: *const c_char) -> tibems_status;
  //admin API
  pub fn tibemsAdmin_Create(admin: *mut usize,
      url: *const c_char, userName: *const c_char,
      password: *const c_char, sslparams: *mut c_void) -> tibems_status;
  pub fn tibemsAdmin_Close(admin: usize) -> tibems_status;
  pub fn tibemsAdmin_GetInfo(
      admin: usize,
      serverInfo: *mut usize) -> tibems_status;
  pub fn tibemsServerInfo_GetQueueCount(
      serverInfo: usize,
      count: *mut usize) -> tibems_status;
  pub fn tibemsDestinationInfo_Create(
      destInfo: *mut usize,
      destName: *const c_char,
      destType: usize) -> tibems_status;
  pub fn tibemsDestinationInfo_Destroy(
      destInfo: usize) -> tibems_status;
  pub fn tibemsAdmin_GetDestination(
      admin: usize,
      destInfo: *mut usize,
      destName: *const c_char,
      destType: usize) -> tibems_status;
  pub fn tibemsDestinationInfo_GetPendingMessageCount(
      destInfo: usize,
      count: *mut i64 ) -> tibems_status;
  pub fn tibemsDestinationInfo_GetConsumerCount(
      destInfo: usize,
      count: &mut usize) -> tibems_status;
  pub fn tibemsDestinationInfo_GetOverflowPolicy(
      destInfo: usize,
      overflowPolicy: *mut usize) -> tibems_status;
  pub fn tibemsAdmin_GetDestinations(
      admin: usize,
      collection: *mut usize,
      pattern: *const c_char,
      destType: tibemsDestinationType,
      permType: tibems_permType,
      statOnly: tibems_bool) ->tibems_status;
  pub fn tibemsAdmin_GetCommandTimeout(
      admin: usize,
      timeout: *mut i64);
  pub fn tibemsAdmin_SetCommandTimeout(
      admin: usize,
      timeout: i64) -> tibems_status;
  pub fn tibemsCollection_GetFirst(
      collection: usize,
      collection_ptr: *mut usize) -> tibems_status;
  pub fn tibemsCollection_GetNext(
      collection: usize,
      collection_ptr: *mut usize) -> tibems_status;
  pub fn tibemsCollection_Destroy(
      collection: usize) -> tibems_status;
  pub fn tibemsDestinationInfo_GetName(
      destInfo: usize,
      name: *const c_char,
      name_len: usize) -> tibems_status;
  pub fn tibemsTopicInfo_GetSubscriptionCount(
      topicInfo: usize,
      count: *mut i64) -> tibems_status;
  pub fn tibemsTopicInfo_GetDurableCount(
      topicInfo: usize,
      count: *mut i64) -> tibems_status;
}

#[allow(dead_code)]
#[repr(C)]
pub struct tibemsErrorContext { pub _private: [u8; 0] }


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
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum tibems_status{
    TIBEMS_OK                          = 0,

    TIBEMS_ILLEGAL_STATE               = 1,
    TIBEMS_INVALID_CLIENT_ID           = 2,
    TIBEMS_INVALID_DESTINATION         = 3,
    TIBEMS_INVALID_SELECTOR            = 4,

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
    TIBEMS_INVALID_ARG                 = 20,

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

    /* SSL related errors */
    TIBEMS_INVALID_CERT                = 150,
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

    TIBEMS_NOT_IMPLEMENTED             = 255
}