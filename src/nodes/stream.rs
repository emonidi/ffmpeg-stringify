use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum StreamType {
    Input,
    Output
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Stream{
    pub path:String,
    pub name:String,
    pub stream_type:StreamType
}
