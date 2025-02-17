use crate::{
    call_response::FuelCallResponse, contract::get_decoded_output,
    execution_script::ExecutableFuelCall, logs::LogDecoder,
};
use fuel_gql_client::fuel_tx::{Output, Receipt, Transaction};
use fuel_tx::Input;
use fuels_core::{
    parameters::{CallParameters, TxParameters},
    Tokenizable,
};
use fuels_signers::{provider::Provider, WalletUnlocked};
use fuels_types::{errors::Error, param_types::ParamType};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
/// Contains all data relevant to a single script call
pub struct ScriptCall {
    pub script_binary: Vec<u8>,
    pub script_data: Vec<u8>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    // This field is not currently used but it will be in the future.
    pub call_parameters: CallParameters,
}

impl ScriptCall {
    pub fn with_outputs(mut self, outputs: Vec<Output>) -> Self {
        self.outputs = outputs;
        self
    }

    pub fn with_inputs(mut self, inputs: Vec<Input>) -> Self {
        self.inputs = inputs;
        self
    }
}

#[derive(Debug)]
#[must_use = "script calls do nothing unless you `call` them"]
/// Helper that handles submitting a script call to a client and formatting the response
pub struct ScriptCallHandler<D> {
    pub script_call: ScriptCall,
    pub tx_parameters: TxParameters,
    pub wallet: WalletUnlocked,
    pub provider: Provider,
    pub output_param: ParamType,
    pub datatype: PhantomData<D>,
    pub log_decoder: LogDecoder,
}

impl<D> ScriptCallHandler<D>
where
    D: Tokenizable + Debug,
{
    pub fn new(
        script_binary: Vec<u8>,
        script_data: Vec<u8>,
        wallet: WalletUnlocked,
        provider: Provider,
        output_param: ParamType,
        log_decoder: LogDecoder,
    ) -> Self {
        let script_call = ScriptCall {
            script_binary,
            script_data,
            inputs: vec![],
            outputs: vec![],
            call_parameters: Default::default(),
        };
        Self {
            script_call,
            tx_parameters: TxParameters::default(),
            wallet,
            provider,
            output_param,
            datatype: PhantomData,
            log_decoder,
        }
    }

    /// Sets the transaction parameters for a given transaction.
    /// Note that this is a builder method, i.e. use it as a chain:
    ///
    /// ```ignore
    /// let params = TxParameters { gas_price: 100, gas_limit: 1000000 };
    /// instance.main(...).tx_params(params).call()
    /// ```
    pub fn tx_params(mut self, params: TxParameters) -> Self {
        self.tx_parameters = params;
        self
    }

    pub fn with_outputs(mut self, outputs: Vec<Output>) -> Self {
        self.script_call = self.script_call.with_outputs(outputs);
        self
    }

    pub fn with_inputs(mut self, inputs: Vec<Input>) -> Self {
        self.script_call = self.script_call.with_inputs(inputs);
        self
    }

    /// Call a script on the node. If `simulate == true`, then the call is done in a
    /// read-only manner, using a `dry-run`. The [`FuelCallResponse`] struct contains the `main`'s value
    /// in its `value` field as an actual typed value `D` (if your method returns `bool`,
    /// it will be a bool, works also for structs thanks to the `abigen!()`).
    /// The other field of [`FuelCallResponse`], `receipts`, contains the receipts of the transaction.
    async fn call_or_simulate(&self, simulate: bool) -> Result<FuelCallResponse<D>, Error> {
        let mut tx = Transaction::script(
            self.tx_parameters.gas_price,
            self.tx_parameters.gas_limit,
            self.tx_parameters.maturity,
            self.script_call.script_binary.clone(),
            self.script_call.script_data.clone(),
            self.script_call.inputs.clone(), // TODO(iqdecay): allow user to set inputs field
            self.script_call.outputs.clone(), // TODO(iqdecay): allow user to set outputs field
            vec![vec![0, 0].into()], //TODO(iqdecay): figure out how to have the right witnesses
        );
        self.wallet.add_fee_coins(&mut tx, 0, 0).await?;

        let tx_execution = ExecutableFuelCall { tx };

        let receipts = if simulate {
            tx_execution.simulate(&self.provider).await?
        } else {
            tx_execution.execute(&self.provider).await?
        };

        self.get_response(receipts)
    }

    /// Call a script on the node, in a state-modifying manner.
    pub async fn call(self) -> Result<FuelCallResponse<D>, Error> {
        Self::call_or_simulate(&self, false).await
    }

    /// Call a script on the node, in a simulated manner, meaning the state of the
    /// blockchain is *not* modified but simulated.
    /// It is the same as the [`call`] method because the API is more user-friendly this way.
    ///
    /// [`call`]: Self::call
    pub async fn simulate(self) -> Result<FuelCallResponse<D>, Error> {
        Self::call_or_simulate(&self, true).await
    }

    /// Create a [`FuelCallResponse`] from call receipts
    pub fn get_response(&self, mut receipts: Vec<Receipt>) -> Result<FuelCallResponse<D>, Error> {
        let token = get_decoded_output(&mut receipts, None, &self.output_param)?;
        Ok(FuelCallResponse::new(
            D::from_token(token)?,
            receipts,
            self.log_decoder.clone(),
        ))
    }
}
