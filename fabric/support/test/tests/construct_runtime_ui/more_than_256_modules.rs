use fabric_support::construct_runtime;

construct_runtime! {
	pub enum Runtime where
		UncheckedExtrinsic = UncheckedExtrinsic,
		Block = Block,
		NodeBlock = Block,
	{
		System: system::{} = 255,
		Noble256: noble256::{},
	}
}

fn main() {}
