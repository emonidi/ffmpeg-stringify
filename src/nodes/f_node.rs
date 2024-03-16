
use serde::{Serialize, Deserialize};

use super::fnode_type::FNodeType;
use super::stream::Stream;
use super::filter::FilterNode;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FNode{
    pub data:FNodeType,
    pub name:String,
   
}

impl From<Stream> for FNode{
    fn from(value: Stream) -> Self {
        Self{

            data: FNodeType::Stream(value.clone()),
            name:value.name
        }
    }
}


impl From<FilterNode> for FNode{
    fn from(value: FilterNode) -> Self {
        Self{
            data:FNodeType::FilterNode(value.clone()),
            name: value.name,
           
        }
    }
}