use dia_oracle_runtime_api::{CoinInfo, TimestampedValue, PriceInfo};
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use pallet_timestamp::Config as TimestampConfig;

pub use dia_oracle_runtime_api::DiaOracleApi as DiaOracleRuntimeApi;


use std::sync::Arc;

#[rpc(client, server)]
pub trait DiaOracleApi<BlockHash, T> where T : TimestampConfig{
	#[method(name = "dia_getCoinInfo")]
	fn get_coin_info(
		&self,
		blockchain: Bytes,
		symbol: Bytes,
		at: Option<BlockHash>,
	) -> RpcResult<TimestampedValue<CoinInfo, T::Moment>>;

	#[method(name = "dia_getValue")]
	fn get_value(
		&self,
		blockchain: Bytes,
		symbol: Bytes,
		at: Option<BlockHash>,
	) -> RpcResult<PriceInfo<T::Moment>>;
}

/// A struct that implements the [`DiaOracleApi`].
pub struct DiaOracleRpc<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> DiaOracleRpc<C, P> {
	/// Create new `TransactionPayment` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, T> DiaOracleApiServer<<Block as BlockT>::Hash, T> for DiaOracleRpc<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: DiaOracleRuntimeApi<Block, T>,
	T : TimestampConfig
{
	fn get_coin_info(
		&self,
		blockchain: Bytes,
		symbol: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<TimestampedValue<CoinInfo, T::Moment>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let r = api
			.get_coin_info(&at, blockchain.to_vec(), symbol.to_vec())
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Unable to query get_coin_info.",
					Some(format!("{:?}", e)),
				))
			})?
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Unable to query get_coin_info.",
					Some(format!("{:?}", e)),
				))
			})?;

		Ok(r)
	}

	fn get_value(
		&self,
		blockchain: Bytes,
		symbol: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<PriceInfo<T::Moment>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let r = api
			.get_value(&at, blockchain.to_vec(), symbol.to_vec())
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Unable to query get_value.",
					Some(format!("{:?}", e)),
				))
			})?
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					"Unable to query get_value.",
					Some(format!("{:?}", e)),
				))
			})?;
		Ok(r)
	}
}
