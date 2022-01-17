use engine::{TransactionEngine, TransactionSource};
use transaction::Transaction;

mod client;
mod engine;
mod transaction;

impl<T: std::io::Read> TransactionSource for csv::Reader<T> {
    fn process_transactions(
        &mut self,
        mut processing_engine: TransactionEngine,
    ) -> std::result::Result<client::Ledger, Box<(dyn std::error::Error)>> {
        // This could be made faster with fewer allocations using the byte_record Iterator instead of the deserialize Iterator,
        // but the tradeoff for that approach is readability. For the sake of clarity, I've opted to deserialize instead
        for result in self.deserialize() {
            let mut txn: Transaction = result?;
            if let Some(amount) = txn.amount.as_mut() {
                if amount.scale() > 4 {
                    amount.rescale(4);
                }
            }
            processing_engine.process(txn);
        }
        Ok(processing_engine.clients)
    }
}

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        return Err("usage: ./transaction-engine <path to file>".to_string());
    }

    let file_path = &args[1];
    let mut transaction_reader = csv::Reader::from_path(file_path).expect("Could not open file");

    match transaction_reader.process_transactions(TransactionEngine::new()) {
        Ok(clients) => {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            for client in clients.0.values() {
                wtr.serialize(client)
                    .expect("Failed to write client to output");
            }
            Ok(())
        }
        Err(e) => Err(format!("Error: {:?}", e)),
    }
}
