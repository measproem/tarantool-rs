// TODO: unify with rollback/commit.rs

use std::{borrow::Cow, io::Write};

use rmpv::Value;

use crate::{
    codec::consts::{keys, RequestType, TransactionIsolationLevel},
    TransportError,
};

use super::RequestBody;

#[derive(Clone, Debug)]
pub struct Begin {
    pub timeout_secs: Option<f64>,
    pub transaction_isolation_level: TransactionIsolationLevel,
}

impl RequestBody for Begin {
    fn request_type() -> RequestType
    where
        Self: Sized,
    {
        RequestType::Begin
    }

    // NOTE: `&mut buf: mut` is required since I don't get why compiler complain
    fn encode(&self, mut buf: &mut dyn Write) -> Result<(), TransportError> {
        let map_len = if self.timeout_secs.is_some() { 2 } else { 1 };
        rmp::encode::write_map_len(&mut buf, map_len)?;
        if let Some(x) = self.timeout_secs {
            rmp::encode::write_pfix(&mut buf, keys::TIMEOUT)?;
            rmp::encode::write_f64(&mut buf, x)?;
        }
        rmp::encode::write_pfix(&mut buf, keys::TXN_ISOLATION)?;
        rmp::encode::write_u8(&mut buf, self.transaction_isolation_level as u8)?;
        Ok(())
    }
}

impl Begin {
    pub fn new(
        timeout_secs: Option<f64>,
        transaction_isolation_level: TransactionIsolationLevel,
    ) -> Self {
        Self {
            timeout_secs,
            transaction_isolation_level,
        }
    }
}
