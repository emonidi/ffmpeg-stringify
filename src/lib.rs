pub(crate) mod nodes;
pub use nodes::*;
use crate::nodes::filter::{FilterOptions,FilterNode, Filter};
use crate::nodes::stream::StreamType;
use crate::nodes::fnode_type::FNodeType;
use crate::nodes::stream::Stream;

pub fn stringify(nodes:Vec<FNodeType>)->String{
    let mut str = "".to_string();
    let inputs:Vec<_> = nodes.clone().into_iter().filter(|node|{
        match node.clone(){
            FNodeType::Stream(node) => {
               if let StreamType::Input = node.stream_type {
                true
               }else{
                false
               }
            },
            _ => {false}
        }

    }).collect();
    inputs.clone().into_iter().for_each(|node|{
        if let FNodeType::Stream(node) = node {
            str = str.clone() + "-i " + node.path.as_str()+" ";
        }
    });

    // println!("{:#?}",graph.into_nodes_edges().1.get(1).unwrap().target());
    str = str + "-filter_complex '";
    let filters:String = nodes.clone().into_iter().filter(|node|{
        match &node.clone() {
            FNodeType::FilterNode(node)=>{
                true
            },
            _=>false
        }
    })
    .map(|node|formatOperation(Into::<FilterNode>::into(node.clone())))
    .collect::<Vec<String>>().join("");

    let output:String = nodes.clone().into_iter().filter(|node|{
        match node.clone(){
            FNodeType::Stream(node) => {
               if let StreamType::Output = node.stream_type {
                true
               }else{
                false
               }
            },
            _ => {false}
        }
    }).map(|node| Into::<Stream>::into(node).path).collect::<Vec<String>>().join("");

    format!("{}{}' {}",str,filters,output)
    
}

pub fn formatFilter(filters:Vec<Filter>)->String{
    let filters:Vec<String> = filters.into_iter().map(|filter|{
       let mut filters = "".to_string();
       match filter.options {
            FilterOptions::HashMap(filter)=>{
             filters = filter.into_iter().map(|option|{
                    return format!("{}={}",option.0,option.1)
                }).collect::<Vec<_>>().join(":")
            },
            FilterOptions::String(filter)=>{
                filters = filter
            }
       }
       format!("{}={}",filter.name,filters)
    }).collect();
    filters.join(",")
}


pub fn formatOperation(filterNode:FilterNode)->String{
    let ins = filterNode.inputs.into_iter().map(|input|{format!("[{}]",input)}).collect::<Vec<String>>().join("");
    let ops = formatFilter(filterNode.filters);
    let outs = filterNode.outputs.into_iter().map(|output|{format!("[{}]",output)}).collect::<Vec<String>>().join("");
    format!("{}{}{}",ins,ops,outs)
}

#[cfg(test)]
mod tests {
    use map_macro::hash_map;
    use super::*;
    use std::vec;
    use crate::nodes::filter::FilterOptions;
    use crate::nodes::stream::StreamType;

    #[test]
    fn serialize(){
        let output = Stream{path:"/data/vid-modified.mp4".to_string(),name:"input1".to_string(), stream_type:StreamType::Output};
        println!("{:#?}",serde_json::to_string(&output).unwrap());
        let overlay = FilterNode{
            filters:vec![
                Filter{
                    name:"fade".to_string(),
                    options:FilterOptions::HashMap(hash_map! {
                        "type".to_string()=>"in".to_string(),
                        "st".to_string()=>"0".to_string(),
                        "duration".to_string()=>"1".to_string()
                    })
                },
                Filter{
                    name:"scale".to_string(),
                    options:FilterOptions::String("512:-2".to_string())
                }
            ],
            
            name: "overlay".to_string(),
            inputs: vec!["0:v".to_string()],
            outputs: vec![],
        };
        println!("{:#?}",serde_json::to_string_pretty(&overlay).unwrap());
    }

    #[test]
    fn it_works() {
        
        let output = Stream{path:"/data/vid-modified.mp4".to_string(),name:"input1".to_string(), stream_type:StreamType::Output};
        let input = Stream{path:"/data/vid.mp4".to_string(),name:"input2".to_string(), stream_type:StreamType::Input};
        // let overlay:FNode = Overlay{x:10,y:10,input:"input1".to_string(),output:"output".to_string(),name:"overlay".to_string(),index:2}.into();
        let overlay = FilterNode{
            filters:vec![
                Filter{
                    name:"fade".to_string(),
                    options:FilterOptions::HashMap(hash_map! {
                        "type".to_string()=>"in".to_string(),
                        "st".to_string()=>"0".to_string(),
                        "duration".to_string()=>"1".to_string()
                    })
                },
                Filter{
                    name:"scale".to_string(),
                    options:FilterOptions::String("512:-2".to_string())
                }
            ],
            
            name: "overlay".to_string(),
            inputs: vec!["0:v".to_string()],
            outputs: vec![],
        };

     
        let string = stringify(vec![FNodeType::Stream(input), FNodeType::FilterNode(overlay),FNodeType::Stream(output)]);
        assert_eq!(string, "-i /data/vid.mp4 -filter_complex '[0:v]fade=type=in:st=0:duration=1,scale=512:-2' /data/vid-modified.mp4".to_string());
    }
}
