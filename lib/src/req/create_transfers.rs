use std::ptr::null_mut;

use tigerbeetle_unofficial_sys::{
    generated_safe::PacketStatusErrorKind, tb_packet_t, tb_transfer_t, TB_OPERATION,
    TB_PACKET_STATUS,
};

use crate::{consts::MAX_TRANSFERS_PER_MSG, resp::create_transfers::CreateTransfersResp, Client};

impl Client {
    /// Caveats:
    /// - those of [`Self::request`] apply
    /// - `accounts.len()` must not exceed [`MAX_TRANSFERS_PER_MSG`]
    pub async fn create_transfers(
        &self,
        transfers: &[tb_transfer_t],
    ) -> Result<CreateTransfersResp, PacketStatusErrorKind> {
        assert!(transfers.len() <= MAX_TRANSFERS_PER_MSG);
        let packet = tb_packet_t {
            operation: TB_OPERATION::TB_OPERATION_CREATE_TRANSFERS as u8,
            status: TB_PACKET_STATUS::TB_PACKET_OK as u8,
            data_size: core::mem::size_of_val(transfers) as u32,
            // cast-safety: request should not modify data but generated bindings take *mut
            data: transfers.as_ptr().cast_mut().cast(),
            // set by [`Req::poll()`]
            user_data: null_mut(),
            // dont-cares?
            next: null_mut(),
            batch_next: null_mut(),
            batch_tail: null_mut(),
            batch_size: 0,
            batch_allowed: 0,
            reserved: [0u8; 7],
        };
        self.request(packet).await.map(CreateTransfersResp)
    }
}
