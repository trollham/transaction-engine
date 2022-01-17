use crate::{
    client::{self, Account},
    transaction::{self, Transaction, TransactionType},
};

pub struct TransactionEngine {
    pub clients: client::Ledger,
    transactions: transaction::Ledger,
}

impl TransactionEngine {
    pub fn new() -> Self {
        TransactionEngine {
            clients: client::Ledger::new(),
            transactions: transaction::Ledger::new(),
        }
    }

    pub fn process(&mut self, txn: Transaction) {
        let client = self
            .clients
            .0
            .entry(client::Id(txn.client))
            .or_insert_with_key(Account::new);

        match txn.txn_type {
            TransactionType::Withdrawal => {
                client.withdraw(&txn.amount.unwrap());
                self.transactions.0.insert(txn.id, txn);
            }
            TransactionType::Deposit => {
                client.deposit(&txn.amount.unwrap());
                self.transactions.0.insert(txn.id, txn);
            }
            TransactionType::Dispute => {
                if let Some(disputed_txn) = self.transactions.0.get_mut(&txn.id) {
                    disputed_txn.dispute();
                    client.dispute(&disputed_txn.amount.unwrap());
                }
            }
            TransactionType::Resolve => match self.transactions.0.get_mut(&txn.id) {
                Some(disputed_txn) if disputed_txn.disputed => {
                    disputed_txn.resolve();
                    client.resolve(&disputed_txn.amount.unwrap());
                }
                _ => {}
            },
            TransactionType::Chargeback => match self.transactions.0.get_mut(&txn.id) {
                Some(disputed_txn) if disputed_txn.disputed => {
                    disputed_txn.resolve();
                    client.chargeback(&disputed_txn.amount.unwrap());
                }
                _ => {}
            },
        }
    }
}

impl Default for TransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TransactionSource {
    fn process_transactions(
        &mut self,
        processing_engine: TransactionEngine,
    ) -> Result<client::Ledger, Box<dyn std::error::Error>>;
}
