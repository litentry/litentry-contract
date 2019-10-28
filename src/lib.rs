#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::{
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    #![env = ink_core::env::DefaultSrmlTypes]

    struct LitentryContract {
        application_name: storage::Value<bool>,
    }

    impl Deploy for LitentryContract {
        fn deploy(&mut self) {
            self.application_name.set(true)
        }
    }

    impl LitentryContract {
        pub(external) fn set(&mut self) {
            *self.application_name = true;
        }
    }
}

