use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::hash::Hash;

use serde::Serialize;

use super::Account;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Id(pub u16);
pub struct Ledger(pub HashMap<Id, Account>);

impl Ledger {
    pub fn new() -> Self {
        Ledger(HashMap::new())
    }
}
