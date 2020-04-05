use serde::{Deserialize, Serialize};
use serde_json::Error;

/// The enum for all the Commands of a player can pass to the server.
/// The corresponding JSON format looks like {"ty":"Move","dat":[1,2]}
///
/// Note, "ty" is short hand for "type" and "dat" => "data"
#[derive(Serialize, Deserialize)]
#[serde(tag = "ty", content = "dat")]
pub enum Command {
    // x, y direction to move
    Move(i32, i32),
    // x, y direction and new tile block id
    ChangeTile(i32, i32, i32),
    MakeSound,
}

#[inline]
pub fn deserialize_command(raw: &str) -> Result<Command, Error> {
    serde_json::from_str::<Command>(raw)
}

#[inline]
pub fn serialize_command(cmd: &Command) -> Result<String, Error> {
    serde_json::to_string(cmd)
}

#[cfg(test)]
mod test {
    use super::{deserialize_command, serialize_command, Command};

    #[test]
    fn test_command_serialize() {
        let raw = "{\"ty\":\"Move\",\"dat\":[1,2]}";
        match deserialize_command(raw).unwrap() {
            Command::Move(x, y) => assert_eq!([x, y], [1, 2]),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_command_deserialize() {
        let cmd = Command::Move(1, 2);
        assert_eq!(
            serialize_command(&cmd).unwrap(),
            "{\"ty\":\"Move\",\"dat\":[1,2]}"
        );

        let cmd = Command::MakeSound;
        assert_eq!(serialize_command(&cmd).unwrap(), "{\"ty\":\"MakeSound\"}");
    }
}
