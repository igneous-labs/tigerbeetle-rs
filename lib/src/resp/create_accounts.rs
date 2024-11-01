use num_traits::FromPrimitive;
use tigerbeetle_unofficial_sys::{
    generated_safe::CreateAccountErrorKind, tb_create_accounts_result_t, TB_CREATE_ACCOUNT_RESULT,
};

use super::RespBuf;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct CreateAccountsResp(pub(crate) RespBuf);

impl CreateAccountsResp {
    #[inline]
    pub const fn as_slice(&self) -> &[tb_create_accounts_result_t] {
        let byte_slice = self.0.as_slice();
        let len = byte_slice.len() / core::mem::size_of::<tb_create_accounts_result_t>();
        unsafe { core::slice::from_raw_parts(byte_slice.as_ptr().cast(), len) }
    }

    /// Yields Ok(index) for successes, Err((index, error)) for failures
    #[inline]
    pub fn iter_results(
        &self,
    ) -> impl Iterator<Item = Result<u32, (u32, CreateAccountErrorKind)>> + '_ {
        self.as_slice().iter().map(
            |tb_create_accounts_result_t { result, index }| match *result {
                TB_CREATE_ACCOUNT_RESULT::TB_CREATE_ACCOUNT_OK => Ok(*index),
                n => Err((*index, CreateAccountErrorKind::from_u32(n).unwrap())),
            },
        )
    }
}

impl AsRef<[tb_create_accounts_result_t]> for CreateAccountsResp {
    #[inline]
    fn as_ref(&self) -> &[tb_create_accounts_result_t] {
        self.as_slice()
    }
}
