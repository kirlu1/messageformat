use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct ClientMessage {
    contents : String
}

#[derive(Serialize, Deserialize)]
struct ServerMessage {
    sender : String,
    contents : String,
}