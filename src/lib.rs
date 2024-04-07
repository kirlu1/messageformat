use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    contents : String
}

#[derive(Serialize, Deserialize)]
pub struct ServerMessage {
    sender : String,
    contents : String,
}