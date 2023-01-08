#![cfg_attr(not(feature = "std"), no_std)]

pub use dia_oracle::{CoinInfo, TimestampedValue, PriceInfo};
use frame_support::sp_std::vec::Vec;
use sp_runtime::DispatchError;
use pallet_timestamp::Config as TimestampConfig;

sp_api::decl_runtime_apis! {
	pub trait DiaOracleApi<T : TimestampConfig>{
		fn get_coin_info(blockchain: Vec<u8>, symbol: Vec<u8>) -> Result<TimestampedValue<CoinInfo, T::Moment>, DispatchError>;
		fn get_value(lockchain: Vec<u8>, symbol: Vec<u8>) -> Result<PriceInfo<T::Moment>,DispatchError>;
	}
}

