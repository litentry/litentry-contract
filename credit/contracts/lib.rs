#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

/// Define the operations to interact with the substrate runtime
/// FetchBalance to get btc and eth balance from Litentry offchain worker
#[ink::chain_extension]
pub trait FetchBalance {
    type ErrorCode = FetchBalanceErr;

    /// Note: this gives the operation a corresponding func_id (1101 in this case),
    /// and the chain-side chain_extension will get the func_id to do further operations.
    #[ink(extension = 1001, returns_result = false)]
    // fn fetch_balance(account_id: <ink_env::DefaultEnvironment as Environment>::AccountId) ->
    fn fetch_balance(account_id: <ink_env::DefaultEnvironment as Environment>::AccountId) -> 
        (Option<<ink_env::DefaultEnvironment as Environment>::Balance>,
        Option<<ink_env::DefaultEnvironment as Environment>::Balance>);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum FetchBalanceErr {
    FailFetchBalance,
}

impl ink_env::chain_extension::FromStatusCode for FetchBalanceErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailFetchBalance),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchBalance;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod credit {
    use super::FetchBalanceErr;

    /// Defines the storage of your contract.
    /// Here we store the credit seed fetched from the chain
    #[ink(storage)]
    pub struct CreditExtension {
        /// Stores a balance value on the storage.
        value: (Option<Balance>, Option<Balance>)
    }

    impl CreditExtension {
        /// Constructor that initializes the value as None.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: (None, None) }
        }

        #[ink(message)]
        pub fn balance_of(&mut self, account_id: AccountId) -> Result<(), FetchBalanceErr> {
            // self.value = self.env().extension().fetch_balance(account_id)?;
            self.value = self.env().extension().fetch_balance(account_id)?;

            Ok(())
        }

        // Simply returns the current value.
        #[ink(message)]
        pub fn get(&self) -> (Option<Balance>, Option<Balance>) {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mut rand_extension = CreditExtension::new();
            // assert_eq!(rand_extension.balance_of(), Ok(()));
            assert_eq!(rand_extension.get(), (None, None));
        }
    }
}
