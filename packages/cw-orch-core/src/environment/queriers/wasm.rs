use cosmwasm_std::{CodeInfoResponse, ContractInfoResponse, HexBinary};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::ChainState,
    CwEnvError,
};

use super::Querier;

pub trait WasmQuerier: Querier {
    type Chain: ChainState;

    fn code_id_hash(&self, code_id: u64) -> Result<HexBinary, Self::Error>;

    /// Query contract info
    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<ContractInfoResponse, Self::Error>;

    /// Query contract state
    fn raw_query(
        &self,
        address: impl Into<String>,
        query_keys: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error>;

    fn smart_query<Q: Serialize, T: DeserializeOwned>(
        &self,
        address: impl Into<String>,
        query_msg: &Q,
    ) -> Result<T, Self::Error>;

    /// Query code
    fn code(&self, code_id: u64) -> Result<CodeInfoResponse, Self::Error>;

    /// Returns the checksum of the WASM file if the env supports it. Will re-upload every time if not supported.
    fn local_hash<T: Uploadable + ContractInstance<Self::Chain>>(
        &self,
        contract: &T,
    ) -> Result<HexBinary, CwEnvError>;

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: impl Into<String>,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error>;
}
