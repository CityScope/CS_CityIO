use std::collections::BTreeMap;
use crate::model::Settable;
use bs58::encode;
use sha2::{Sha256, Digest};

pub type Tree = BTreeMap<String, String>;

impl Settable for Tree{
    fn domain_prefix() -> String {
        "tree".to_string()
    }
    fn id(&self) -> String {
      let cat = self.iter().map(|(k,v)|{
          format!("{} {}", k, v)
      }).collect::<Vec<String>>().join(" ");
      encode(Sha256::digest(cat.as_bytes())).into_string()
    }
}


