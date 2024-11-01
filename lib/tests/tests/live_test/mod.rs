//! Make sure to have a cluster running on port 3000 with the state set to what the tests expect
//! (see each individual test's docs for more info on this)

#![cfg(feature = "live-test")]

mod create_accounts;
mod create_transfers;
mod get_account_transfers;
mod init;
mod lookup_accounts;
mod lookup_transfers;

pub fn live_test_client() -> tigerbeetle_unofficial::Client {
    tigerbeetle_unofficial::Client::init(0, "127.0.0.1:3000").unwrap()
}
