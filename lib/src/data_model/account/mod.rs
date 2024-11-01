use std::borrow::Borrow;

use tigerbeetle_unofficial_sys::{generated_safe::AccountFlags, tb_account_t};

use super::{
    HasCode, HasId, HasLedger, HasTimestamp, HasUserDataU128, HasUserDataU32, HasUserDataU64,
};

pub trait HasAccountFlags {
    fn account_flags(&self) -> AccountFlags;
}

#[inline]
pub fn account_to_create<
    A: Borrow<
        impl HasId
            + HasUserDataU128
            + HasUserDataU64
            + HasUserDataU32
            + HasLedger
            + HasCode
            + HasTimestamp
            + HasAccountFlags,
    >,
>(
    acc: A,
) -> tb_account_t {
    let acc = acc.borrow();
    tb_account_t {
        id: acc.id().into(),
        user_data_128: acc.user_data_128(),
        user_data_64: acc.user_data_64(),
        user_data_32: acc.user_data_32(),
        ledger: acc.ledger().into(),
        code: acc.code().into(),
        flags: acc.account_flags().bits(),
        timestamp: acc.timestamp(),
        // all of these fields must be 0 or creation will error
        debits_pending: 0,
        debits_posted: 0,
        credits_pending: 0,
        credits_posted: 0,
        reserved: 0,
    }
}

// Transforms a sequence of [`AccountToCreate`] into a `Vec` ready to be passed to [`crate::Client::create_accounts`]
#[inline]
pub fn accounts_to_create<
    A: Borrow<
        impl HasId
            + HasUserDataU128
            + HasUserDataU64
            + HasUserDataU32
            + HasLedger
            + HasCode
            + HasTimestamp
            + HasAccountFlags,
    >,
>(
    itr: impl IntoIterator<Item = A>,
) -> Vec<tb_account_t> {
    itr.into_iter().map(account_to_create).collect()
}
