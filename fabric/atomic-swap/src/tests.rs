#![cfg(test)]

use super::*;
use crate as noble_atomic_swap;

use fabric_support::parameter_types;
use tet_core::H256;
use tp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = fabric_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = fabric_system::mocking::MockBlock<Test>;

fabric_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: fabric_system::{Module, Call, Config, Storage, Event<T>},
		Balances: noble_balances::{Module, Call, Storage, Config<T>, Event<T>},
		AtomicSwap: noble_atomic_swap::{Module, Call, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: fabric_system::limits::BlockWeights =
		fabric_system::limits::BlockWeights::simple_max(1024);
}
impl fabric_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = Call;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type NobleInfo = NobleInfo;
	type AccountData = noble_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl noble_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}
parameter_types! {
	pub const ProofLimit: u32 = 1024;
	pub const ExpireDuration: u64 = 100;
}
impl Config for Test {
	type Event = Event;
	type SwapAction = BalanceSwapAction<u64, Balances>;
	type ProofLimit = ProofLimit;
}

const A: u64 = 1;
const B: u64 = 2;

pub fn new_test_ext() -> tet_io::TestExternalities {
	let mut t = fabric_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = noble_balances::GenesisConfig::<Test> {
		balances: vec![
			(A, 100),
			(B, 200),
		],
	};
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}

#[test]
fn two_party_successful_swap() {
	let mut chain1 = new_test_ext();
	let mut chain2 = new_test_ext();

	// A generates a random proof. Keep it secret.
	let proof: [u8; 2] = [4, 2];
	// The hashed proof is the blake2_256 hash of the proof. This is public.
	let hashed_proof = blake2_256(&proof);

	// A creates the swap on chain1.
	chain1.execute_with(|| {
		AtomicSwap::create_swap(
			Origin::signed(A),
			B,
			hashed_proof.clone(),
			BalanceSwapAction::new(50),
			1000,
		).unwrap();

		assert_eq!(Balances::free_balance(A), 100 - 50);
		assert_eq!(Balances::free_balance(B), 200);
	});

	// B creates the swap on chain2.
	chain2.execute_with(|| {
		AtomicSwap::create_swap(
			Origin::signed(B),
			A,
			hashed_proof.clone(),
			BalanceSwapAction::new(75),
			1000,
		).unwrap();

		assert_eq!(Balances::free_balance(A), 100);
		assert_eq!(Balances::free_balance(B), 200 - 75);
	});

	// A reveals the proof and claims the swap on chain2.
	chain2.execute_with(|| {
		AtomicSwap::claim_swap(
			Origin::signed(A),
			proof.to_vec(),
			BalanceSwapAction::new(75),
		).unwrap();

		assert_eq!(Balances::free_balance(A), 100 + 75);
		assert_eq!(Balances::free_balance(B), 200 - 75);
	});

	// B use the revealed proof to claim the swap on chain1.
	chain1.execute_with(|| {
		AtomicSwap::claim_swap(
			Origin::signed(B),
			proof.to_vec(),
			BalanceSwapAction::new(50),
		).unwrap();

		assert_eq!(Balances::free_balance(A), 100 - 50);
		assert_eq!(Balances::free_balance(B), 200 + 50);
	});
}
