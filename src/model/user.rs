use bs58::encode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Debug;
use crate::model::Settable;

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialUser {
    pub nickname: String,
    pub email: String,
}

impl From<PartialUser> for User {
    fn from(p_user: PartialUser) -> User {
        User::new(p_user.nickname, p_user.email)
    }
}

impl Settable for User {
    fn domain_prefix() -> String {
        String::from("user")
    }

    fn id(&self) -> String {
        self.id.to_string()
    }

    fn list_item(&self) -> String {
        serde_json::to_string(&vec![&self.id, &self.nickname])
            .expect("User-Settable should be Serializable")
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    id: String,
    pub nickname: String,
    pub is_verified: bool,
}

impl User {
    pub fn new(nickname: String, email: String) -> Self {
        let cat = format!("email:{}", &email);
        let id = encode(Sha256::digest(&cat.as_bytes())).into_string();

        Self {
            id,
            nickname,
            is_verified: false,
        }
    }

    pub fn json(&self) -> String {
        serde_json::to_string(self).expect("user should be able to serialize")
    }
}
