//! This is a facade over Substrate's democracy pallet


#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{self as system, ensure_root, RawOrigin};
use frame_support::{decl_error, decl_event, decl_module, dispatch,
                    traits::{Currency, ReservableCurrency, LockableCurrency, Get, EnsureOrigin},
                    weights::{Weight, DispatchClass}};
use sp_runtime::{print, traits::{Hash}};
use pallet_democracy::{BalanceOf, ReferendumIndex};

#[cfg(test)]
mod tests;

// TODO: Lot of additions and removals needed.

// type BalanceOf<T> = <<T as democracy::Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait + pallet_democracy::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event {
        CouncilMemberAdded,
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        DUmmy,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        // TODO: Fix weight
        #[weight = 0]
		fn propose(origin, proposal_hash: T::Hash, #[compact] value: BalanceOf<T>) {
            <pallet_democracy::Module<T>>::propose(origin, proposal_hash, value)?;
		}

		// TODO: Fix weight
        #[weight = 0]
		pub fn council_propose(origin, proposal_hash: T::Hash) {
			<pallet_democracy::Module<T>>::external_propose_majority(origin, proposal_hash)?;
		}

        #[weight = T::MaximumBlockWeight::get()]
		pub fn enact_proposal(origin, proposal_hash: T::Hash, index: ReferendumIndex) -> dispatch::DispatchResult {
			<pallet_democracy::Module<T>>::enact_proposal(origin, proposal_hash, index)
		}

        // TODO: Fix weight
        #[weight = (0, DispatchClass::Operational)]
		fn cancel_queued(origin, which: ReferendumIndex) {
			<pallet_democracy::Module<T>>::cancel_queued(origin, which)?;
		}

        // TODO: Fix weight
        #[weight = 0]
        fn clear_public_proposals(origin) {
			<pallet_democracy::Module<T>>::clear_public_proposals(origin)?;
		}

		// TODO: Set weight
		fn on_initialize(n: T::BlockNumber) -> Weight {
			<pallet_democracy::Module<T>>::on_initialize(n)
		}
    }
}