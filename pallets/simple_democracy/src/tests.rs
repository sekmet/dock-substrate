#![cfg(test)]

use super::*;

use frame_support::{
    assert_err, assert_ok, impl_outer_origin, parameter_types,
    sp_runtime::{
        testing::{Header, UintAuthorityId},
        traits::{BlakeTwo256, ConvertInto, IdentityLookup, OpaqueKeys},
        ConsensusEngineId, KeyTypeId, Perbill,
    },
    traits::FindAuthor,
    weights::{constants::WEIGHT_PER_SECOND, Weight},
};
use frame_system::{self as system, RawOrigin, EnsureOneOf, EnsureRoot};
use sp_core::{crypto::key_types, H256};

impl_outer_origin! {
    pub enum Origin for TestRuntime {}
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct TestRuntime;

type DemocracyModule = Module<TestRuntime>;

type System = system::Module<TestRuntime>;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const TransactionByteFee: Balance = 1;
}

impl system::Trait for TestRuntime {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * MaximumBlockWeight::get();
}

impl pallet_scheduler::Trait for Test {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<u64>;
    type MaxScheduledPerBlock = ();
    type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Trait for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
	pub const LaunchPeriod: u64 = 2;
	pub const VotingPeriod: u64 = 2;
	pub const FastTrackVotingPeriod: u64 = 2;
	pub const MinimumDeposit: u64 = 1;
	pub const EnactmentPeriod: u64 = 2;
	pub const CooloffPeriod: u64 = 2;
	pub const MaxVotes: u32 = 100;
}

thread_local! {
	static PREIMAGE_BYTE_DEPOSIT: RefCell<u64> = RefCell::new(0);
	static INSTANT_ALLOWED: RefCell<bool> = RefCell::new(false);
}
pub struct PreimageByteDeposit;
impl Get<u64> for PreimageByteDeposit {
    fn get() -> u64 { PREIMAGE_BYTE_DEPOSIT.with(|v| *v.borrow()) }
}

impl super::Trait for Test {
    type Proposal = Call;
    type Event = Event;
    type Currency = pallet_balances::Module<Self>;
    type EnactmentPeriod = EnactmentPeriod;
    type LaunchPeriod = LaunchPeriod;
    type VotingPeriod = VotingPeriod;
    type FastTrackVotingPeriod = FastTrackVotingPeriod;
    type MinimumDeposit = MinimumDeposit;
    type ExternalOrigin = EnsureSignedBy<Two, u64>;
    type ExternalMajorityOrigin = EnsureSignedBy<Three, u64>;
    type ExternalDefaultOrigin = EnsureSignedBy<One, u64>;
    type FastTrackOrigin = EnsureSignedBy<Five, u64>;
    type CancellationOrigin = EnsureSignedBy<Four, u64>;
    type CooloffPeriod = CooloffPeriod;
    type PreimageByteDeposit = PreimageByteDeposit;
    type Slash = ();
    type InstantOrigin = EnsureSignedBy<Six, u64>;
    type InstantAllowed = InstantAllowed;
    type Scheduler = Scheduler;
    type MaxVotes = MaxVotes;
    type PalletsOrigin = OriginCaller;
    type WeightInfo = ();
}
