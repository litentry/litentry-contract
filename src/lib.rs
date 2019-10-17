#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::{
    memory::format,
    storage,
    litentry,
};
use ink_lang::contract;

contract! {
    #![env = ink_core::env::DefaultSrmlTypes]

    struct LitentryContract {
        application_name: storage::Value<String>,
    }

    impl Deploy for LitentryContract {
        fn deploy(&mut self) {
            self.application_name.set("Motel")
        }
    }

    impl LitentryContract {
        pub(external) fn issus_token(&mut self, token_hash: storage::Value<Hash>) {
            litentry::issue_token(token_hash);
        }

        pub(external) fn verify_signature(&self, verify_signature) -> bool {
            litentry::verify_signature(token_hash);
        }
    }
}

