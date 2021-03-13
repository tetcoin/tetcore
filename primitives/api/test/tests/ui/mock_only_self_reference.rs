use tetcore_test_runtime_client::runtime::Block;

tp_api::decl_runtime_apis! {
	pub trait Api {
		fn test(data: u64);
		fn test2(data: u64);
	}
}

struct MockApi;

tp_api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		fn test(self, data: u64) {}

		fn test2(&mut self, data: u64) {}
	}
}

fn main() {}
