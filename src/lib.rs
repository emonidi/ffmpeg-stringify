pub(crate) mod nodes;
use std::fmt::Debug;

use crate::nodes::filter::{Filter, FilterNode, FilterOptions};
use crate::nodes::fnode_type::FNodeType;
use crate::nodes::stream::Stream;
use crate::nodes::stream::StreamType;
pub use nodes::*;

pub fn stringify(nodes: Vec<FNodeType>) -> String {
    let mut str = "".to_string();
    let inputs: Vec<_> = nodes
        .clone()
        .into_iter()
        .filter(|node| match node.clone() {
            FNodeType::Stream(node) => {
                if let StreamType::Input = node.stream_type {
                    true
                } else {
                    false
                }
            }
            _ => false,
        })
        .collect();
    inputs.clone().into_iter().for_each(|node| {
        if let FNodeType::Stream(node) = node {
            str = str.clone() + "-i " + node.path.as_str() + " ";
        }
    });

    // println!("{:#?}",graph.into_nodes_edges().1.get(1).unwrap().target());
    str = str + "-filter_complex '";
    let filters: String = nodes
        .clone()
        .into_iter()
        .filter(|node| match &node.clone() {
            FNodeType::FilterNode(node) => true,
            _ => false,
        })
        .map(|node| formatOperation(Into::<FilterNode>::into(node.clone())))
        .collect::<Vec<String>>()
        .join(";");

    let filtered_output: Vec<_> = nodes
        .clone()
        .into_iter()
        .filter(|node| match node.clone() {
            FNodeType::Stream(node) => {
                if let StreamType::Output = node.stream_type {
                    true
                } else {
                    false
                }
            }
            _ => false,
        })
        .collect();

    let mut maps = String::from("");
    filtered_output.clone().into_iter().for_each(|node| {
        let n: Stream = node.into();
        if let Some(inputs) = n.inputs {
            inputs
                .into_iter()
                .for_each(|input| maps = format!("{} -map '[{}]'", maps, input));
        }
    });

    let output = filtered_output
        .clone()
        .into_iter()
        .map(|node| Into::<Stream>::into(node).path)
        .collect::<Vec<String>>()
        .join("");

    format!("{}{}' {} {}", str, filters, maps, output)
}

pub fn format_filter(filters: Vec<Filter>) -> String {
    let filters: Vec<String> = filters
        .into_iter()
        .map(|filter| {
            let mut filters = "".to_string();
            match filter.options {
                FilterOptions::HashMap(filter) => {
                    filters = filter
                        .into_iter()
                        .map(|option| return format!("{}={}", option.0, option.1))
                        .collect::<Vec<_>>()
                        .join(":")
                }
                FilterOptions::String(filter) => filters = filter,
            }
            format!("{}={}", filter.name, filters)
        })
        .collect();
    filters.join(",")
}

pub fn formatOperation(filterNode: FilterNode) -> String {
    let ins = filterNode
        .inputs
        .into_iter()
        .map(|input| format!("[{}]", input))
        .collect::<Vec<String>>()
        .join("");
    let ops = format_filter(filterNode.filters);
    let outs = filterNode
        .outputs
        .into_iter()
        .map(|output| format!("[{}]", output))
        .collect::<Vec<String>>()
        .join("");
    format!("{}{}{}", ins, ops, outs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::filter::FilterOptions;
    use crate::nodes::stream::StreamType;
    use map_macro::hash_map;
    use std::{path, vec};

    #[test]
    fn serialize() {
        let output = Stream {
            path: "/data/vid-modified.mp4".to_string(),
            name: "input1".to_string(),
            stream_type: StreamType::Output,
            inputs: None,
        };
        println!("{:#?}", serde_json::to_string(&output).unwrap());
        let overlay = FilterNode {
            filters: vec![Filter {
                name: "fade".to_string(),
                options: FilterOptions::HashMap(hash_map! {
                    "type".to_string()=>"in".to_string(),
                    "st".to_string()=>"0".to_string(),
                    "duration".to_string()=>"1".to_string()
                }),
            }],

            name: "overlay".to_string(),
            inputs: vec!["0:v".to_string()],
            outputs: vec![],
        };
        // println!("{:#?}",serde_json::to_string_pretty(&overlay).unwrap());
    }

    #[test]
    fn it_works() {
        let output = Stream {
            path: "./data/vid-modified.mp4".to_string(),
            name: "input1".to_string(),
            stream_type: StreamType::Output,
            inputs: Some(vec!["audio".to_string(), "out0".to_string()]),
        };
        let input = Stream {
            path: "./data/vid.mp4".to_string(),
            name: "input2".to_string(),
            stream_type: StreamType::Input,
            inputs: None,
        };
        let sound_input: Stream = Stream {
            path: "./data/sound.mp3".to_string(),
            name: "sound_input".to_string(),
            stream_type: StreamType::Input,
            inputs: None,
        };
        let trim = FilterNode {
            filters: vec![Filter {
                name: "trim".to_string(),
                options: FilterOptions::HashMap(hash_map! {
                    "duration".to_string()=>"4".to_string(),
                }),
            }],

            name: "trim".to_string(),
            inputs: vec!["0:v".to_string()],
            outputs: vec!["out0".to_string()],
        };

        let audio = FilterNode {
            filters: vec![Filter {
                name: "volume".to_string(),
                options: FilterOptions::String("1".to_string()),
            }],
            name: "volume".to_string(),
            outputs: vec!["audio".to_string()],
            inputs: vec!["trimmed_audio".to_string()],
        };

        let audio_trim = FilterNode {
            filters: vec![Filter {
                name: "atrim".to_string(),
                options: FilterOptions::HashMap(hash_map! {
                    "duration".to_string()=>"4".to_string()
                }),
            }],
            name: "atrim".to_string(),
            inputs: vec!["1:a".to_string()],
            outputs: vec!["trimmed_audio".to_string()],
        };

        let string = stringify(vec![
            FNodeType::Stream(input),
            FNodeType::Stream(sound_input),
            FNodeType::FilterNode(trim),
            FNodeType::FilterNode(audio_trim),
            FNodeType::FilterNode(audio),
            FNodeType::Stream(output),
        ]);
        println!("{:?}", string);
        assert_eq!(string, "-i /data/vid.mp4 -filter_complex '[0:v]fade=type=in:st=0:duration=1,scale=512:-2' /data/vid-modified.mp4".to_string());
    }
}
