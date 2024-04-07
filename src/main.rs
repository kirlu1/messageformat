use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}


#[derive(Serialize, Deserialize)]
struct ClientMessage {
    contents : String
}

#[derive(Serialize, Deserialize)]
struct ServerMessage {
    sender : String,
    contents : String,
}