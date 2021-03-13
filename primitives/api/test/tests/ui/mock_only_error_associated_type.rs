use tetcore_test_runtime_client::runtime::Block;

tp_api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
	}
}

struct MockApi;

tp_api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		type OtherData = u32;

		fn test(data: u64) {}
	}
}

fn main() {}
