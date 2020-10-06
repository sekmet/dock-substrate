use super::*;

use super::Call as MigrateCall;
use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use frame_support::{
    assert_err, assert_ok, impl_outer_dispatch, impl_outer_origin, parameter_types,
    weights::{constants::WEIGHT_PER_SECOND, DispatchClass, DispatchInfo, Weight},
};
use frame_system::{self as system, RawOrigin};
use sp_core::H256;

impl_outer_origin! {
    pub enum Origin for TestRuntime {}
}

impl_outer_dispatch! {
    pub enum Call for TestRuntime where origin: Origin {
        system::System,
        token_migration::MigrationModule,
    }
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct TestRuntime;

type MigrationModule = Module<TestRuntime>;
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
    type Call = Call;
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

impl balances::Trait for TestRuntime {
    type Balance = u64;
    type DustRemoval = ();
    type Event = ();
    type ExistentialDeposit = ();
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

impl Trait for TestRuntime {
    type Event = ();
    type Currency = balances::Module<Self>;
}

fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap()
        .into()
}

#[test]
fn add_migrator() {
    new_test_ext().execute_with(|| {
        let acc_1 = 1;
        assert_ok!(MigrationModule::add_migrator(
            RawOrigin::Root.into(),
            acc_1,
            30
        ));
        assert_err!(
            MigrationModule::add_migrator(RawOrigin::Root.into(), acc_1, 30),
            Error::<TestRuntime>::MigratorAlreadyPresent
        );
        assert_eq!(MigrationModule::migrators(&acc_1).unwrap(), 30);
    })
}

#[test]
fn remove_migrator() {
    new_test_ext().execute_with(|| {
        let acc_1 = 1;
        assert_err!(
            MigrationModule::remove_migrator(RawOrigin::Root.into(), acc_1),
            Error::<TestRuntime>::UnknownMigrator
        );
        MigrationModule::add_migrator(RawOrigin::Root.into(), acc_1, 30).unwrap();
        assert_ok!(MigrationModule::remove_migrator(
            RawOrigin::Root.into(),
            acc_1
        ));
    });
}

#[test]
fn expand_migrator() {
    new_test_ext().execute_with(|| {
        let acc_1 = 1;
        assert_err!(
            MigrationModule::expand_migrator(RawOrigin::Root.into(), acc_1, 10),
            Error::<TestRuntime>::UnknownMigrator
        );
        MigrationModule::add_migrator(RawOrigin::Root.into(), acc_1, 10).unwrap();
        assert_ok!(MigrationModule::expand_migrator(
            RawOrigin::Root.into(),
            acc_1,
            35
        ));
        assert_eq!(MigrationModule::migrators(&acc_1).unwrap(), 45);
        // Overflow check
        assert_err!(
            MigrationModule::expand_migrator(RawOrigin::Root.into(), acc_1, 65500),
            Error::<TestRuntime>::CannotExpandMigrator
        );
    });
}

#[test]
fn contract_migrator() {
    new_test_ext().execute_with(|| {
        let acc_1 = 1;
        assert_err!(
            MigrationModule::contract_migrator(RawOrigin::Root.into(), acc_1, 10),
            Error::<TestRuntime>::UnknownMigrator
        );
        MigrationModule::add_migrator(RawOrigin::Root.into(), acc_1, 10).unwrap();
        assert_ok!(MigrationModule::contract_migrator(
            RawOrigin::Root.into(),
            acc_1,
            5
        ));
        assert_eq!(MigrationModule::migrators(&acc_1).unwrap(), 5);
        // Underflow check
        assert_err!(
            MigrationModule::contract_migrator(RawOrigin::Root.into(), acc_1, 6),
            Error::<TestRuntime>::CannotContractMigrator
        );
    });
}

#[test]
fn migrate() {
    new_test_ext().execute_with(|| {
        let recip_acc_1 = 1;
        let recip_acc_2 = 2;
        let recip_acc_3 = 3;
        let recip_acc_4 = 4;
        let recip_acc_5 = 5;
        let migrator_acc = 10;

        let _ = <TestRuntime as Trait>::Currency::deposit_creating(&migrator_acc, 100);
        MigrationModule::add_migrator(RawOrigin::Root.into(), migrator_acc, 4).unwrap();

        // No of recipients more than allowed migrations
        let mut recips_1 = BTreeMap::new();
        recips_1.insert(recip_acc_1, 10);
        recips_1.insert(recip_acc_2, 1);
        recips_1.insert(recip_acc_3, 50);
        recips_1.insert(recip_acc_4, 30);
        recips_1.insert(recip_acc_5, 2);
        assert_err!(
            MigrationModule::migrate(RawOrigin::Signed(migrator_acc).into(), recips_1),
            Error::<TestRuntime>::ExceededMigrations
        );
        assert_eq!(MigrationModule::migrators(&migrator_acc).unwrap(), 4);

        let mut recips_2 = BTreeMap::new();
        recips_2.insert(recip_acc_1, 10);
        recips_2.insert(recip_acc_2, 1);
        assert_ok!(MigrationModule::migrate(
            RawOrigin::Signed(migrator_acc).into(),
            recips_2
        ));
        assert_eq!(MigrationModule::migrators(&migrator_acc).unwrap(), 2);
        assert_eq!(
            <TestRuntime as Trait>::Currency::free_balance(&migrator_acc).saturated_into::<u64>(),
            89
        );

        // Insufficient balance of migrator
        let mut recips_3 = BTreeMap::new();
        recips_3.insert(recip_acc_1, 85);
        recips_3.insert(recip_acc_2, 5);
        assert!(
            MigrationModule::migrate(RawOrigin::Signed(migrator_acc).into(), recips_3).is_err()
        );
        assert_eq!(MigrationModule::migrators(&migrator_acc).unwrap(), 2);
        assert_eq!(
            <TestRuntime as Trait>::Currency::free_balance(&migrator_acc).saturated_into::<u64>(),
            89
        );

        let mut recips_4 = BTreeMap::new();
        recips_4.insert(recip_acc_1, 85);
        recips_4.insert(recip_acc_2, 4);
        assert_ok!(MigrationModule::migrate(
            RawOrigin::Signed(migrator_acc).into(),
            recips_4
        ));
        assert_eq!(MigrationModule::migrators(&migrator_acc).unwrap(), 0);
        assert_eq!(
            <TestRuntime as Trait>::Currency::free_balance(&migrator_acc).saturated_into::<u64>(),
            0
        );

        // TODO: Check for overflow as well
    });
}

#[test]
fn signed_extension_test() {
    // Check that the signed extension `OnlyMigrator` only allows registered migrator
    new_test_ext().execute_with(|| {
        // Migrators
        let migrator_acc_1 = 1;
        let migrator_acc_2 = 2;
        let migrator_acc_3 = 3;

        // Register migrators and fuel them
        let _ = <TestRuntime as Trait>::Currency::deposit_creating(&migrator_acc_1, 100);
        let _ = <TestRuntime as Trait>::Currency::deposit_creating(&migrator_acc_2, 90);
        MigrationModule::add_migrator(RawOrigin::Root.into(), migrator_acc_1, 4).unwrap();
        MigrationModule::add_migrator(RawOrigin::Root.into(), migrator_acc_2, 5).unwrap();

        let signed_extension = OnlyMigrator::<TestRuntime>(PhantomData);

        // The call made by migrator. The recipients being empty is irrelevant for this test.
        let call: <TestRuntime as system::Trait>::Call =
            Call::MigrationModule(MigrateCall::migrate(BTreeMap::new()));

        let tx_info = DispatchInfo {
            weight: 3,
            class: DispatchClass::Normal,
            pays_fee: Pays::No,
        };

        // Registered migrators should not pass signed extension
        assert!(signed_extension
            .validate(&migrator_acc_1, &call, &tx_info, 20)
            .is_ok());
        assert!(signed_extension
            .validate(&migrator_acc_2, &call, &tx_info, 20)
            .is_ok());

        // Unregistered migrator should not pass signed extension
        assert!(signed_extension
            .validate(&migrator_acc_3, &call, &tx_info, 20)
            .is_err());

        MigrationModule::add_migrator(RawOrigin::Root.into(), migrator_acc_3, 6).unwrap();

        assert!(signed_extension
            .validate(&migrator_acc_3, &call, &tx_info, 20)
            .is_ok());

        assert_ok!(MigrationModule::remove_migrator(
            RawOrigin::Root.into(),
            migrator_acc_1
        ));

        assert!(signed_extension
            .validate(&migrator_acc_1, &call, &tx_info, 20)
            .is_err());
    });
}
