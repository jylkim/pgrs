trait MessageCode {}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum RequestCode {
    Query,
}

impl MessageCode for RequestCode {}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ResponseCode {
    Complete,
    Error,
}

impl MessageCode for ResponseCode {}

#[derive(serde::Serialize, serde::Deserialize)]
struct Message<T: MessageCode> {
    pub code: T,
    pub payload: String,
}

pub type Request = Message<RequestCode>;
pub type Response = Message<ResponseCode>;
