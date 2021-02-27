use crate::model::Settable;
use serde_json::Value;
use sha2::{Digest, Sha256};
use bs58::encode;

pub type Blob = Value;

impl Settable for Blob {
    fn domain_prefix() -> String {
        "blob".to_string()
    }

    fn id(&self) -> String {
        let slice = serde_json::to_vec(&self)
            .expect("Value should be able to Serialize.");
        encode(Sha256::digest(&slice)).into_string()
    }

}

