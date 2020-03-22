use serde::{Deserialize, Serialize};
use serde_json as json;

/// Client request
/// https://serde.rs/enum-representations.html#internally-tagged
#[derive(Serialize, Deserialize)]
#[rtype(result = "()")]
#[serde(tag = "cmd", content = "data")]
pub enum CommandRequest {
    // Move a player
    Move
}

/// Server response

