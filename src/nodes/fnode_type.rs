use serde::{Serialize, Deserialize};

use crate::nodes::stream::Stream;

use super::filter::FilterNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FNodeType{
    Stream(Stream),
    FilterNode(FilterNode),
   
}

impl From<FNodeType> for FilterNode {
    fn from(value: FNodeType) -> Self {
       if let FNodeType::FilterNode(value) = value {
            value
       }else{
        panic!("Not a FilterNode")
       }
    }
}


impl From<FNodeType> for Stream {
    fn from(value: FNodeType) -> Self {
         if let FNodeType::Stream(value) = value {
            value
         }else{
            panic!("Not a Stream")
         }
    }
}
