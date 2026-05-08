use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct NodeMessage{
    pub node_id:String,
    pub message:String,
}