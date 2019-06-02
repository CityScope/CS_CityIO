use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_derive::{Serialize, Deserialize};
use serde_json::{Map, Value};

use sha256::sha256::{format_hash, hash};

pub type JSONState = Arc<Mutex<HashMap<String, Value>>>;
pub type JSONObject = Map<String, Value>;

#[derive(Serialize, Deserialize)]
pub struct TableList(Vec<String>);

#[derive(Serialize, Deserialize)]
pub struct Meta {
    id: String,
    timestamp: u64,
    apiv: String,
}

impl Meta {
    pub fn new(dump: &str) -> Meta {
        let id = format_hash(&hash(&dump));
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let apiv = String::from("2.0");

        Meta {
            id: id,
            timestamp: now,
            apiv: apiv,
        }
    }

    pub fn id(&self) -> String {
        self.id.to_owned()
    }
}
