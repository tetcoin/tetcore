use fabric_support::construct_runtime;

construct_runtime! {
	pub enum Runtime where
		UncheckedExtrinsic = UncheckedExtrinsic,
		Block = Block,
		NodeBlock = Block,
	{
		System: system::{} = 5,
		Noble1: noble1::{} = 3,
		Noble2: noble2::{},
		Noble3: noble3::{},
	}
}

fn main() {}
