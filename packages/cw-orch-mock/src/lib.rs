//! Integration testing execution environment backed by a [cw-multi-test](cw_multi_test) App.
//! It has an associated state that stores deployment information for easy retrieval and contract interactions.

// Export our fork
pub extern crate cw_multi_test;

mod bech32;
mod core;
pub mod queriers;
mod simple;
mod state;

pub use self::core::{Mock, MockBech32, MockBase};

pub type MockApp = self::core::MockApp<MockApi>;
pub type MockAppBech32 = self::core::MockApp<MockApiBech32>;

pub mod base {
    pub use super::core::MockBase;
}

use cosmwasm_std::testing::MockApi;
use cw_multi_test::addons::MockApiBech32;
pub use state::MockState;
