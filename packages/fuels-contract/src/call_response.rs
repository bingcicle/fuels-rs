use crate::logs::LogDecoder;
use fuel_tx::Receipt;
use fuels_core::{Parameterize, Tokenizable};
use fuels_types::errors::Error;

/// [`FuelCallResponse`] is a struct that is returned by a call to the contract or script. Its value
/// field holds the decoded typed value returned by the contract's method. The other field holds all
/// the receipts returned by the call.
//
/// The name is `FuelCallResponse` instead of `CallResponse` because it would be ambiguous with the
/// `CALL` opcode.
#[derive(Debug)]
// ANCHOR: fuel_call_response
pub struct FuelCallResponse<D> {
    pub value: D,
    pub receipts: Vec<Receipt>,
    pub gas_used: u64,
    pub log_decoder: LogDecoder,
}
// ANCHOR_END: fuel_call_response

impl<D> FuelCallResponse<D> {
    /// Get the gas used from ScriptResult receipt
    fn get_gas_used(receipts: &[Receipt]) -> u64 {
        receipts
            .iter()
            .rfind(|r| matches!(r, Receipt::ScriptResult { .. }))
            .expect("could not retrieve ScriptResult")
            .gas_used()
            .expect("could not retrieve gas used from ScriptResult")
    }

    pub fn new(value: D, receipts: Vec<Receipt>, log_decoder: LogDecoder) -> Self {
        Self {
            value,
            gas_used: Self::get_gas_used(&receipts),
            receipts,
            log_decoder,
        }
    }

    pub fn get_logs(&self) -> Result<Vec<String>, Error> {
        self.log_decoder.get_logs(&self.receipts)
    }

    pub fn get_logs_with_type<T: Tokenizable + Parameterize>(&self) -> Result<Vec<T>, Error> {
        self.log_decoder.get_logs_with_type::<T>(&self.receipts)
    }
}
