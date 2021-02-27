// mod user;
mod table;
mod module;
mod tree;
mod blob;

use serde::Serialize;
use std::fmt::Debug;

pub use module::Module;
pub use tree::Tree;
pub use blob::Blob;
pub use table::{TempTable, Table};

pub trait Settable: Serialize + Debug {
    fn domain_prefix() -> String;

    fn prefix(&self) -> String {
        Self::domain_prefix()
    } //FIXME: theres got be a smarter way...

    fn id(&self) -> String;

    fn domain(&self) -> String {
        return format!("{}:{}", Self::domain_prefix(), &self.id());
    }

    fn json(&self)-> String {
        serde_json::to_string(&self).expect("I should be Serialize-able")
    }
}

