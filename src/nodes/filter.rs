use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum  FilterOptions {
    HashMap(HashMap<String,String>),
    String(String)
}
#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Filter{
    pub name:String, 
    pub options:FilterOptions
}




#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilterNode{
    pub name:String,
    pub inputs:Vec<String>,
    pub outputs:Vec<String>,
    pub filters:Vec<Filter>
}

