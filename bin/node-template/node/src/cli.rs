use structopt::StructOpt;
use tc_cli::RunCmd;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Key management cli utilities
	Key(tc_cli::KeySubcommand),
	/// Build a chain specification.
	BuildSpec(tc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(tc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(tc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(tc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(tc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(tc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(tc_cli::RevertCmd),

	/// The custom benchmark subcommmand benchmarking runtime nobles.
	#[structopt(name = "benchmark", about = "Benchmark runtime nobles.")]
	Benchmark(fabric_benchmarking_cli::BenchmarkCmd),
}
