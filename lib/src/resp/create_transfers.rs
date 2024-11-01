use num_traits::FromPrimitive;
use tigerbeetle_unofficial_sys::{
    generated_safe::CreateTransferErrorKind, tb_create_transfers_result_t,
    TB_CREATE_TRANSFER_RESULT,
};

use super::RespBuf;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct CreateTransfersResp(pub(crate) RespBuf);

impl CreateTransfersResp {
    #[inline]
    pub const fn as_slice(&self) -> &[tb_create_transfers_result_t] {
        let byte_slice = self.0.as_slice();
        let len = byte_slice.len() / core::mem::size_of::<tb_create_transfers_result_t>();
        unsafe { core::slice::from_raw_parts(byte_slice.as_ptr().cast(), len) }
    }

    /// Yields Ok(index) for successes, Err((index, error)) for failures
    #[inline]
    pub fn iter_results(
        &self,
    ) -> impl Iterator<Item = Result<u32, (u32, CreateTransferErrorKind)>> + '_ {
        self.as_slice().iter().map(
            |tb_create_transfers_result_t { result, index }| match *result {
                TB_CREATE_TRANSFER_RESULT::TB_CREATE_TRANSFER_OK => Ok(*index),
                n => Err((*index, CreateTransferErrorKind::from_u32(n).unwrap())),
            },
        )
    }
}

impl AsRef<[tb_create_transfers_result_t]> for CreateTransfersResp {
    #[inline]
    fn as_ref(&self) -> &[tb_create_transfers_result_t] {
        self.as_slice()
    }
}
