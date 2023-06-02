use thiserror::Error;

use libsrt_sys as srt;

use std::{
    convert::From,
    io::{self, ErrorKind},
    os::raw::c_int,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum SrtError {
    #[error("Internal error when setting the right error code")]
    Unknown,
    #[error("The value set when the last error was cleared and no error has occurred since then")]
    Success,
    #[error("General setup error resulting from internal system state")]
    ConnSetup,
    #[error("Connection timed out while attempting to connect to the remote address")]
    NoServer,
    #[error("Connection has been rejected: {0:?}")]
    ConnRej(SrtRejectReason),
    #[error(
        "An error occurred when trying to call a system function on an internally used UDP socket"
    )]
    SockFail,
    #[error("A possible tampering with the handshake packets was detected, or encryption request wasn't properly fulfilled.")]
    SecFail,
    #[error("A socket that was vital for an operation called in blocking mode has been closed during the operation")]
    Closed,
    #[error("General connection failure of unknown details")]
    ConnFail,
    #[error("The socket was properly connected, but the connection has been broken")]
    ConnLost,
    #[error("The socket is not connected")]
    NoConn,
    #[error("System or standard library error reported unexpectedly for unknown purpose")]
    Resource,
    #[error("System was unable to spawn a new thread when requried")]
    Thread,
    #[error("System was unable to allocate memory for buffers")]
    NoBuf,
    #[error("System was unable to allocate system specific objects")]
    SysObj,
    #[error("General filesystem error (for functions operating with file transmission)")]
    File,
    #[error("Failure when trying to read from a given position in the file")]
    InvRdOff,
    #[error("Read permission was denied when trying to read from file")]
    RdPerm,
    #[error("Failed to set position in the written file")]
    InvWrOff,
    #[error("Write permission was denied when trying to write to a file")]
    WrPerm,
    #[error("Invalid operation performed for the current state of a socket")]
    InvOp,
    #[error("The socket is currently bound and the required operation cannot be performed in this state")]
    BoundSock,
    #[error("The socket is currently connected and therefore performing the required operation is not possible")]
    ConnSock,
    #[error("Call parameters for API functions have some requirements that were not satisfied")]
    InvParam,
    #[error("The API function required an ID of an entity (socket or group) and it was invalid")]
    InvSock,
    #[error(
        "The operation to be performed on a socket requires that it first be explicitly bound"
    )]
    UnboundSock,
    #[error("The socket passed for the operation is required to be in the listen state")]
    NoListen,
    #[error(
        "The required operation cannot be performed when the socket is set to rendezvous mode"
    )]
    RdvNoServ,
    #[error("An attempt was made to connect to a socket set to rendezvous mode that was not first bound")]
    RdvUnbound,
    #[error("The function was used incorrectly in the message API")]
    InvalMsgApi,
    #[error("The function was used incorrectly in the stream (buffer) API")]
    InvalBufferApi,
    #[error("The port tried to be bound for listening is already busy")]
    DupListen,
    #[error("Size exceeded")]
    LargeMsg,
    #[error("The epoll ID passed to an epoll function is invalid")]
    InvPollId,
    #[error("The epoll container currently has no subscribed sockets")]
    PollEmpty,
    #[error("General asynchronous failure (not in use currently)")]
    AsyncFail,
    #[error("Sending operation is not ready to perform")]
    AsyncSnd,
    #[error("Receiving operation is not ready to perform")]
    AsyncRcv,
    #[error("The operation timed out")]
    Timeout,
    #[error(
        "With SRTO_TSBPDMODE and SRTO_TLPKTDROP set to true, some packets were dropped by sender"
    )]
    Congest,
    #[error("Receiver peer is writing to a file that the agent is sending")]
    PeerErr,
}

pub fn handle_result<T>(ok: T, return_code: i32) -> Result<T, SrtError> {
    match return_code {
        0 => Ok(ok),
        -1 => Err(get_last_error()),
        e => unreachable!("unrecognized return code {}", e),
    }
}

pub fn get_last_error() -> SrtError {
    let mut _errno_loc = 0;
    let err_no = unsafe { srt::srt_getlasterror(&mut _errno_loc as *mut c_int) };
    let err = srt::SRT_ERRNO(err_no);
    SrtError::from(err)
}

impl From<SrtError> for io::Error {
    fn from(e: SrtError) -> Self {
        io::Error::new(
            match e {
                SrtError::Unknown => ErrorKind::Other,
                SrtError::Success => ErrorKind::Other,
                SrtError::ConnSetup => ErrorKind::ConnectionRefused,
                SrtError::NoServer => ErrorKind::ConnectionRefused,
                SrtError::ConnRej(_) => ErrorKind::ConnectionRefused,
                SrtError::SockFail => ErrorKind::AddrNotAvailable,
                SrtError::SecFail => ErrorKind::ConnectionRefused,
                SrtError::ConnFail => ErrorKind::ConnectionRefused,
                SrtError::Closed => ErrorKind::AddrNotAvailable,
                SrtError::ConnLost => ErrorKind::ConnectionAborted,
                SrtError::NoConn => ErrorKind::NotConnected,
                SrtError::Resource => ErrorKind::Other,
                SrtError::Thread => ErrorKind::Other,
                SrtError::NoBuf => ErrorKind::Other,
                SrtError::SysObj => ErrorKind::Other,
                SrtError::File => ErrorKind::NotFound,
                SrtError::InvRdOff => ErrorKind::InvalidInput,
                SrtError::RdPerm => ErrorKind::PermissionDenied,
                SrtError::InvWrOff => ErrorKind::InvalidInput,
                SrtError::WrPerm => ErrorKind::PermissionDenied,
                SrtError::InvOp => ErrorKind::InvalidInput,
                SrtError::BoundSock => ErrorKind::AddrInUse,
                SrtError::ConnSock => ErrorKind::AddrInUse,
                SrtError::InvParam => ErrorKind::InvalidInput,
                SrtError::InvSock => ErrorKind::AddrNotAvailable,
                SrtError::UnboundSock => ErrorKind::NotConnected,
                SrtError::NoListen => ErrorKind::InvalidInput,
                SrtError::RdvNoServ => ErrorKind::ConnectionRefused,
                SrtError::RdvUnbound => ErrorKind::ConnectionRefused,
                SrtError::InvalMsgApi => ErrorKind::InvalidInput,
                SrtError::InvalBufferApi => ErrorKind::InvalidInput,
                SrtError::DupListen => ErrorKind::AddrInUse,
                SrtError::LargeMsg => ErrorKind::Other,
                SrtError::InvPollId => ErrorKind::AddrNotAvailable,
                SrtError::PollEmpty => ErrorKind::Other,
                SrtError::AsyncFail => ErrorKind::WouldBlock,
                SrtError::AsyncSnd => ErrorKind::WouldBlock,
                SrtError::AsyncRcv => ErrorKind::WouldBlock,
                SrtError::Timeout => ErrorKind::TimedOut,
                SrtError::Congest => ErrorKind::Other,
                SrtError::PeerErr => ErrorKind::Other,
            },
            e,
        )
    }
}

impl From<srt::SRT_ERRNO> for SrtError {
    fn from(err_no: srt::SRT_ERRNO) -> Self {
        match err_no {
            srt::SRT_ERRNO::SRT_EUNKNOWN => SrtError::Unknown,
            srt::SRT_ERRNO::SRT_SUCCESS => SrtError::Success,
            srt::SRT_ERRNO::SRT_ECONNSETUP => SrtError::ConnSetup,
            srt::SRT_ERRNO::SRT_ENOSERVER => SrtError::NoServer,
            srt::SRT_ERRNO::SRT_ECONNREJ => SrtError::ConnRej(SrtRejectReason::Unknown),
            srt::SRT_ERRNO::SRT_ESOCKFAIL => SrtError::SockFail,
            srt::SRT_ERRNO::SRT_ESECFAIL => SrtError::SecFail,
            srt::SRT_ERRNO::SRT_ESCLOSED => SrtError::Closed,
            srt::SRT_ERRNO::SRT_ECONNFAIL => SrtError::ConnFail,
            srt::SRT_ERRNO::SRT_ECONNLOST => SrtError::ConnLost,
            srt::SRT_ERRNO::SRT_ENOCONN => SrtError::NoConn,
            srt::SRT_ERRNO::SRT_ERESOURCE => SrtError::Resource,
            srt::SRT_ERRNO::SRT_ETHREAD => SrtError::Thread,
            srt::SRT_ERRNO::SRT_ENOBUF => SrtError::NoBuf,
            srt::SRT_ERRNO::SRT_ESYSOBJ => SrtError::SysObj,
            srt::SRT_ERRNO::SRT_EFILE => SrtError::File,
            srt::SRT_ERRNO::SRT_EINVRDOFF => SrtError::InvRdOff,
            srt::SRT_ERRNO::SRT_ERDPERM => SrtError::RdPerm,
            srt::SRT_ERRNO::SRT_EINVWROFF => SrtError::InvWrOff,
            srt::SRT_ERRNO::SRT_EWRPERM => SrtError::WrPerm,
            srt::SRT_ERRNO::SRT_EINVOP => SrtError::InvOp,
            srt::SRT_ERRNO::SRT_EBOUNDSOCK => SrtError::BoundSock,
            srt::SRT_ERRNO::SRT_ECONNSOCK => SrtError::ConnSock,
            srt::SRT_ERRNO::SRT_EINVPARAM => SrtError::InvParam,
            srt::SRT_ERRNO::SRT_EINVSOCK => SrtError::InvSock,
            srt::SRT_ERRNO::SRT_EUNBOUNDSOCK => SrtError::UnboundSock,
            srt::SRT_ERRNO::SRT_ENOLISTEN => SrtError::NoListen,
            srt::SRT_ERRNO::SRT_ERDVNOSERV => SrtError::RdvNoServ,
            srt::SRT_ERRNO::SRT_ERDVUNBOUND => SrtError::RdvUnbound,
            srt::SRT_ERRNO::SRT_EINVALMSGAPI => SrtError::InvalMsgApi,
            srt::SRT_ERRNO::SRT_EINVALBUFFERAPI => SrtError::InvalBufferApi,
            srt::SRT_ERRNO::SRT_EDUPLISTEN => SrtError::DupListen,
            srt::SRT_ERRNO::SRT_ELARGEMSG => SrtError::LargeMsg,
            srt::SRT_ERRNO::SRT_EINVPOLLID => SrtError::InvPollId,
            srt::SRT_ERRNO::SRT_EPOLLEMPTY => SrtError::PollEmpty,
            srt::SRT_ERRNO::SRT_EASYNCFAIL => SrtError::AsyncFail,
            srt::SRT_ERRNO::SRT_EASYNCSND => SrtError::AsyncSnd,
            srt::SRT_ERRNO::SRT_EASYNCRCV => SrtError::AsyncRcv,
            srt::SRT_ERRNO::SRT_ETIMEOUT => SrtError::Timeout,
            srt::SRT_ERRNO::SRT_ECONGEST => SrtError::Congest,
            srt::SRT_ERRNO::SRT_EPEERERR => SrtError::PeerErr,
            _ => unreachable!("unrecognized error no"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SrtRejectReason {
    Unknown,    // initial set when in progress
    System,     // broken due to system function error
    Peer,       // connection was rejected by peer
    Resource,   // internal problem with resource allocation
    Rogue,      // incorrect data in handshake messages
    Backlog,    // listener's backlog exceeded
    IPE,        // internal program error
    Close,      // socket is closing
    Version,    // peer is older version than agent's minimum set
    RdvCookie,  // rendezvous cookie collision
    BadSecret,  // wrong password
    Unsecure,   // password required or unexpected
    MessageAPI, // streamapi/messageapi collision
    Congestion, // incompatible congestion-controller type
    Filter,     // incompatible packet filter
    Group,      // incompatible group
    Timeout,    // connection timeout
}

impl From<srt::SRT_REJECT_REASON> for SrtRejectReason {
    fn from(reject_reason: srt::SRT_REJECT_REASON) -> Self {
        match reject_reason {
            srt::SRT_REJECT_REASON::SRT_REJ_UNKNOWN => SrtRejectReason::Unknown, // initial set when in progress
            srt::SRT_REJECT_REASON::SRT_REJ_SYSTEM => SrtRejectReason::System,
            srt::SRT_REJECT_REASON::SRT_REJ_PEER => SrtRejectReason::Peer,
            srt::SRT_REJECT_REASON::SRT_REJ_RESOURCE => SrtRejectReason::Resource,
            srt::SRT_REJECT_REASON::SRT_REJ_ROGUE => SrtRejectReason::Rogue,
            srt::SRT_REJECT_REASON::SRT_REJ_BACKLOG => SrtRejectReason::Backlog,
            srt::SRT_REJECT_REASON::SRT_REJ_IPE => SrtRejectReason::IPE,
            srt::SRT_REJECT_REASON::SRT_REJ_CLOSE => SrtRejectReason::Close,
            srt::SRT_REJECT_REASON::SRT_REJ_VERSION => SrtRejectReason::Version,
            srt::SRT_REJECT_REASON::SRT_REJ_RDVCOOKIE => SrtRejectReason::RdvCookie,
            srt::SRT_REJECT_REASON::SRT_REJ_BADSECRET => SrtRejectReason::BadSecret,
            srt::SRT_REJECT_REASON::SRT_REJ_UNSECURE => SrtRejectReason::Unsecure,
            srt::SRT_REJECT_REASON::SRT_REJ_MESSAGEAPI => SrtRejectReason::MessageAPI,
            srt::SRT_REJECT_REASON::SRT_REJ_CONGESTION => SrtRejectReason::Congestion,
            srt::SRT_REJECT_REASON::SRT_REJ_FILTER => SrtRejectReason::Filter,
            srt::SRT_REJECT_REASON::SRT_REJ_GROUP => SrtRejectReason::Group,
            srt::SRT_REJECT_REASON::SRT_REJ_TIMEOUT => SrtRejectReason::Timeout,
            _ => unreachable!("unrecognized SRT_REJECT_REASON"),
        }
    }
}
