use tetcore_test_runtime_client::runtime::Block;
use tp_api::ApiError;

sp_api::decl_runtime_apis! {
	pub trait Api {
		fn test();
	}
}

struct MockApi;

sp_api::mock_impl_runtime_apis! {
	impl Api<Block> for MockApi {
		#[advanced]
		fn test(&self) -> Result<tet_core::NativeOrEncoded<()>, ApiError> {
			Ok(().into())
		}
	}
}

fn main() {}
