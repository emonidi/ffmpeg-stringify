use std::fmt::Error;

use crate::nodes::fnode_type::FNodeType;
use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum StreamType {
    Input,
    Output
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Stream{
    pub(crate) path:String,
    pub(crate) name:String,
    pub(crate) stream_type:StreamType
}
