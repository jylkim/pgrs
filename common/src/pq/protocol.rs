pub trait MessageCode {}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum RequestCode {
    Query,
}

impl MessageCode for RequestCode {}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum ResponseCode {
    Complete,
    Error,
}

impl MessageCode for ResponseCode {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Message<T: MessageCode> {
    pub code: T,
    pub payload: String,
}

pub type RequestMessage = Message<RequestCode>;
pub type ResponseMessage = Message<ResponseCode>;
