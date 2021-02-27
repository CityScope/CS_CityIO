use serde_json::Value;
use bs58::encode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::model::Settable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    name: String,
    pub data: Value
}

impl Settable for Module{
    fn domain_prefix() -> String {
        "module".to_string()
    }

    fn id(&self) -> String {
        let raw = serde_json::to_vec(&self.data)
            .expect("serde_json::Value should be Serializedable");
        encode(Sha256::digest(&raw)).into_string()
    }

}

impl<'a> Settable for &'a Module {
    fn domain_prefix() -> String {
        "module".to_string()
    }

    fn id(&self) -> String {
        let raw = serde_json::to_vec(&self.data)
            .expect("serde_json::Value should be Serializedable");
        encode(Sha256::digest(&raw)).into_string()
    }

}

impl Module{
    pub fn new(name: &str, data: &Value) -> Self {
        Self{
            name: name.to_string(),
            data: data.to_owned()
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn data(&self) -> Value {
        self.data.to_owned()
    }
}


