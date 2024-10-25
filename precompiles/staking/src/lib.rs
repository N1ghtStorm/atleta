#![cfg_attr(not(feature = "std"), no_std)]
#![allow(missing_docs)]

use fp_evm::PrecompileHandle;
use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use pallet_evm::{AddressMapping, PrecompileFailure};
use precompile_utils::prelude::*;
use sp_core::{Decode, Get, H160, U256};
use sp_runtime::traits::{Dispatchable, StaticLookup};
use sp_std::{marker::PhantomData, vec::Vec};

pub struct StakingPrecompile<Runtime>(PhantomData<Runtime>);

type BalanceOf<Runtime> = <Runtime as pallet_staking::Config>::CurrencyBalance;

#[precompile_utils::precompile]
impl<Runtime> StakingPrecompile<Runtime>
where
    Runtime: pallet_evm::Config + pallet_staking::Config,
    Runtime::AccountId: Into<H160>,
    <Runtime as pallet_staking::Config>::CurrencyBalance: Into<U256> + TryFrom<U256>,
    Runtime::Lookup: StaticLookup<Source = Runtime::AccountId>,
    Runtime::RuntimeCall: From<pallet_staking::Call<Runtime>>,
    <Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
    Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
{
    #[precompile::public("activeEra()")]
    #[precompile::view]
    fn active_era(_: &mut impl PrecompileHandle) -> EvmResult<u32> {
        // TODO: record gas
        let era_info = pallet_staking::Pallet::<Runtime>::active_era()
            .ok_or_else(|| Self::custom_err("Unable to get active era"))?;
        Ok(era_info.index)
    }

    #[precompile::public("sessionsPerEra()")]
    #[precompile::view]
    fn sessions_per_era(_: &mut impl PrecompileHandle) -> EvmResult<u32> {
        Ok(<Runtime as pallet_staking::Config>::SessionsPerEra::get())
    }

    #[precompile::public("slashingSpans(address)")]
    fn slashing_spans(
        _: &mut impl PrecompileHandle,
        address: Address,
    ) -> EvmResult<(u32, u32, u32, Vec<u32>)> {
        let addr = Runtime::AddressMapping::into_account_id(address.0);
        let pallet_staking::slashing::SlashingSpans { .. } =
            pallet_staking::SlashingSpans::<Runtime>::get(addr)
                .ok_or_else(|| Self::custom_err("Unable to get slashing spans"))?;
        // XXX: SlashingSpans fields are private
        unimplemented!()
    }

    #[precompile::public("erasTotalStake(uint32)")]
    #[precompile::view]
    fn eras_total_stake(_: &mut impl PrecompileHandle, era: u32) -> EvmResult<U256> {
        let total = pallet_staking::ErasTotalStake::<Runtime>::get(era);
        Ok(total.into())
    }

    #[precompile::public("erasValidatorReward(uint32)")]
    fn eras_validator_reward(_: &mut impl PrecompileHandle, era: u32) -> EvmResult<U256> {
        let reward = pallet_staking::ErasValidatorReward::<Runtime>::get(era)
            .ok_or_else(|| Self::custom_err("Unable to get eras validator reward"))?;
        Ok(reward.into())
    }

    #[precompile::public("nominate(address[])")]
    fn nominate(h: &mut impl PrecompileHandle, targets: Vec<Address>) -> EvmResult<()> {
        let targets = targets
            .into_iter()
            .map(|addr| addr.0)
            .map(Runtime::AddressMapping::into_account_id)
            .map(|addr| {
                Runtime::Lookup::lookup(addr)
                    .map_err(|_| Self::custom_err("Unable to lookup address"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let call = pallet_staking::Call::<Runtime>::nominate { targets };
        let origin = Some(Runtime::AddressMapping::into_account_id(h.context().caller));
        RuntimeHelper::<Runtime>::try_dispatch(h, origin.into(), call)?;
        Ok(())
    }

    #[precompile::public("bond(uint256,uint8)")]
    fn bond(h: &mut impl PrecompileHandle, value: U256, payee: u8) -> EvmResult<()> {
        let value = Self::u256_to_amount(value)?;
        let payee = pallet_staking::RewardDestination::decode(&mut &[payee][..])
            .map_err(|_| Self::custom_err("Unable to decode RewardDestination variant"))?;

        let call = pallet_staking::Call::<Runtime>::bond { value, payee };
        let origin = Some(Runtime::AddressMapping::into_account_id(h.context().caller));
        RuntimeHelper::<Runtime>::try_dispatch(h, origin.into(), call)?;
        Ok(())
    }

    #[precompile::public("bond(uint256,address)")]
    fn bond_into(h: &mut impl PrecompileHandle, value: U256, address: Address) -> EvmResult<()> {
        let value = Self::u256_to_amount(value)?;
        let payee = pallet_staking::RewardDestination::Account(
            Runtime::AddressMapping::into_account_id(address.0),
        );

        let call = pallet_staking::Call::<Runtime>::bond { value, payee };
        let origin = Some(Runtime::AddressMapping::into_account_id(h.context().caller));
        RuntimeHelper::<Runtime>::try_dispatch(h, origin.into(), call)?;
        Ok(())
    }

    fn u256_to_amount(value: U256) -> MayRevert<BalanceOf<Runtime>> {
        value
            .try_into()
            .map_err(|_| RevertReason::value_is_too_large("amount type").into())
    }

    fn custom_err(reason: &'static str) -> PrecompileFailure {
        PrecompileFailure::Error { exit_status: evm::ExitError::Other(reason.into()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACC_LEN: usize = 20;
    type AccountId = [u8; ACC_LEN];
    type RewardDestination = pallet_staking::RewardDestination<AccountId>;

    #[test]
    #[allow(deprecated)]
    #[allow(clippy::redundant_pattern_matching)]
    fn it_deserializes_reward_destination_from_single_byte() {
        assert_eq!(RewardDestination::decode(&mut &[0u8][..]), Ok(RewardDestination::Staked));
        assert_eq!(RewardDestination::decode(&mut &[1u8][..]), Ok(RewardDestination::Stash));
        assert_eq!(RewardDestination::decode(&mut &[2u8][..]), Ok(RewardDestination::Controller));
        assert_eq!(RewardDestination::decode(&mut &[4u8][..]), Ok(RewardDestination::None));

        assert!(matches!(RewardDestination::decode(&mut &[3u8][..]), Err(_)));

        let account = [42; ACC_LEN];
        let mut bytes = [0u8; ACC_LEN + 1];
        bytes[0] = 3; // account
        bytes[1..].copy_from_slice(&account[..]);
        assert_eq!(
            RewardDestination::decode(&mut &bytes[..]),
            Ok(RewardDestination::Account(account))
        );
    }
}
