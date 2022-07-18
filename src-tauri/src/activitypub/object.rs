use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityPub {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "type")]
    type_object: String,
    #[serde(rename = "actor")]
    actor: String,
    #[serde(rename = "to")]
    to: Vec<String>,
    object: Object,
}

#[derive(Serialize, Deserialize, Debug)]
struct Object {
    #[serde(rename = "@context")]
    context: HashMap<String, String>,
    id: String,
    #[serde(rename = "type")]
    object_type: String,
    #[serde(rename = "attributedTo")]
    attributed_to: String,
    content: String,
}

#[tauri::command]
pub fn test() {
    let object = r#"
        {
            "@context": "https://www.w3.org/ns/activitystreams",
            "type": "Like",
            "actor": "https://example.net/~mallory",
            "to": ["https://hatchat.example/sarah/",
                    "https://example.com/peeps/john/"],
            "object": {
                "@context": {"@language": "en"},
                "id": "https://example.org/~alice/note/23",
                "type": "Note",
                "attributedTo": "https://example.org/~alice",
                "content": "I'm a goat"
            }
        }
    "#;

    println!("{:?}", serde_json::from_str::<ActivityPub>(object));
}

/*
{
  "@context": "https://www.w3.org/ns/activitystreams",
  "type": "Like",
  "actor": "https://example.net/~mallory",
  "to": ["https://hatchat.example/sarah/",
         "https://example.com/peeps/john/"],
  "object": {
    "@context": {"@language": "en"},
    "id": "https://example.org/~alice/note/23",
    "type": "Note",
    "attributedTo": "https://example.org/~alice",
    "content": "I'm a goat"
  }
}
 */
