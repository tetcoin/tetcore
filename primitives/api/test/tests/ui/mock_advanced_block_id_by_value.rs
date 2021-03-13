use tetcore_test_runtime_client::runtime::Block;
use tp_api::ApiError;

tp_api::decl_runtime_apis! {
	pub trait Api {
		fn test();
	}
}

struct MockApi;

tp_api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		#[advanced]
		fn test(&self, _: BlockId<Block>) -> Result<tet_core::NativeOrEncoded<()>, ApiError> {
			Ok(().into())
		}
	}
}

fn main() {}
