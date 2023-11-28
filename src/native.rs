
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub mod defetch_native {

	use std::fs;

	pub struct DefetchPre;

	impl DefetchPre {
		pub async fn defetch ( url: String ) -> (Vec<u8>, i32) {
			return match fs::read(url) {
				Ok (bytes) => (bytes, 200),
				Err (err) => {
					return (Vec::new(), 404);
				}
			}
		}
	}

}
