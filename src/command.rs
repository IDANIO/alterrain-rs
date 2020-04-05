use serde::{Deserialize, Serialize};

/// The enum for all the Commands of a player can pass to the server.
/// The corresponding JSON format looks like
///
/// Note, "ty" is short hand for "type" and "dat" => "data"
#[derive(Serialize, Deserialize)]
#[serde(tag = "ty", content = "dat")]
enum Command {
    // x, y direction to move
    Move(i32, i32),
    // x, y direction and new tile block id
    ChangeTile(i32, i32, i32),
    MakeSound,
}

#[cfg(test)]
mod test {
    use super::Command;

    #[test]
    fn test_command_serialize() {
        let raw = "{\"ty\":\"Move\",\"dat\":[1,2]}";
        match serde_json::from_str::<Command>(raw).unwrap() {
            Command::Move(x, y) => assert_eq!([x, y], [1, 2]),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_command_deserialize() {
        let cmd = Command::Move(1, 2);
        assert_eq!(
            serde_json::to_string(&cmd).unwrap(),
            "{\"ty\":\"Move\",\"dat\":[1,2]}"
        );

        let cmd = Command::MakeSound;
        assert_eq!(
            serde_json::to_string(&cmd).unwrap(),
            "{\"ty\":\"MakeSound\"}"
        );
    }
}
