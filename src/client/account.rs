use rust_decimal::Decimal;
use serde::{ser::SerializeStruct, Serialize};

use super::Id;

#[derive(Debug)]
pub struct Account {
    pub id: Id,
    pub(crate) available: Decimal,
    pub(crate) held: Decimal,
    pub(crate) locked: bool,
}

impl Account {
    pub fn new(id: &Id) -> Self {
        Account {
            id: Id(id.0),
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            locked: false,
        }
    }

    pub fn total(&self) -> Decimal {
        self.available + self.held
    }

    /*
       It's not necessarily safe for these to fail silently as they do, but since there's no mention of interactivity (no one needs to be told something failed),
       I omitted return types to avoid a glut of "Ignored Result" warnings from the compiler
    */
    pub fn deposit(&mut self, amount: &Decimal) {
        // The specification does not mention how to handle accounts that are locked and trying to deposit/withdraw, so I'm assuming that a locked account should not be mutable

        if !self.locked {
            self.available += amount;
        }
    }

    pub fn withdraw(&mut self, amount: &Decimal) {
        // The specification does not mention how to handle accounts that are locked and trying to deposit/withdraw, so I'm assuming that a locked account should not be mutable
        if !self.locked && amount <= &self.available {
            self.available -= amount;
        }
    }

    pub fn dispute(&mut self, amount: &Decimal) {
        self.held += amount;
        self.available -= amount;
    }

    pub fn resolve(&mut self, amount: &Decimal) {
        self.held -= amount;
        self.available += amount;
    }

    pub fn chargeback(&mut self, amount: &Decimal) {
        self.held -= amount;
        self.locked = true;
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ClientAccount", 5)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &self.available)?;
        state.serialize_field("held", &self.held)?;
        state.serialize_field("total", &self.total())?;
        state.serialize_field("locked", &self.locked)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Id;

    use super::Account;
    use rust_decimal_macros::dec;

    #[test]
    pub fn depositing_works() {
        let mut test_account = Account::new(&Id(1));
        test_account.deposit(&dec!(100));
        test_account.deposit(&dec!(50));
        test_account.deposit(&dec!(10));

        assert_eq!(dec!(160), test_account.available);
    }

    #[test]
    pub fn withdrawing_works() {
        let mut test_account = Account::new(&Id(1));
        test_account.deposit(&dec!(100));
        test_account.withdraw(&dec!(80));

        assert_eq!(dec!(20), test_account.available);
    }

    #[test]
    pub fn withdrawing_more_than_available_fails() {
        let mut test_account = Account::new(&Id(1));
        test_account.deposit(&dec!(100));
        test_account.withdraw(&dec!(101));
        assert_eq!(dec!(100), test_account.available);
    }

    #[test]
    pub fn dispute() {
        let mut test_account = Account::new(&Id(1));
        test_account.deposit(&dec!(100));
        test_account.dispute(&dec!(20));

        assert_eq!(dec!(80), test_account.available);
        assert_eq!(dec!(20), test_account.held);
    }

    #[test]
    pub fn resolve() {
        let mut test_account = Account::new(&Id(1));
        test_account.deposit(&dec!(100));
        test_account.dispute(&dec!(20));

        test_account.resolve(&dec!(20));
        assert_eq!(dec!(100), test_account.available);
        assert_eq!(dec!(0), test_account.held);
    }

    #[test]
    pub fn chargeback() {
        let mut test_account = Account::new(&Id(1));
        test_account.deposit(&dec!(100));
        test_account.dispute(&dec!(20));
        test_account.chargeback(&dec!(20));

        assert_eq!(dec!(80), test_account.available);
        assert_eq!(dec!(0), test_account.held);
        assert!(test_account.locked);
    }
}
