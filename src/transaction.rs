use std::collections::HashMap;

use rust_decimal::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Withdrawal,
    Deposit,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    // `type` is a resereved word, we need a different name for this member
    #[serde(rename = "type")]
    pub txn_type: TransactionType,
    // id is a nicer name than `tx`
    #[serde(rename = "tx")]
    pub id: Id,
    pub client: u16,
    pub amount: Option<Decimal>,
    #[serde(skip)]
    pub disputed: bool,
}

impl Transaction {
    pub fn dispute(&mut self) {
        self.disputed = true
    }

    pub fn resolve(&mut self) {
        self.disputed = false
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct Id(pub u32);

pub struct Ledger(pub HashMap<Id, Transaction>);

impl Ledger {
    pub fn new() -> Self {
        Ledger(HashMap::new())
    }
}
