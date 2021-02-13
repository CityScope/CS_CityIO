mod user;
mod table;

use serde::Serialize;
use std::fmt::Debug;

pub use user::{User, PartialUser};
pub use table::{Table, PartialTable};

pub trait Settable: Serialize + Debug {
    fn domain_prefix() -> String;

    fn prefix(&self) -> String {
        Self::domain_prefix()
    } //FIXME: theres got be a smarter way...

    fn id(&self) -> String;
    fn list_item(&self) -> String;

    fn domain(&self) -> String {
        return format!("{}:{}", Self::domain_prefix(), &self.id());
    }

    fn json(&self) -> String {
        serde_json::to_string(&self).expect("I should be Serialize-able")
    }
}

