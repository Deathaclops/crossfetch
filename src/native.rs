
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
pub mod crossfetch_native {
	use std::fs;
	pub struct CrossFetchPre;
	impl CrossFetchPre {
		pub async fn crossfetch ( path: String ) -> (Option<Vec<u8>>, i32) {
			return match fs::read(path) {
				Ok (bytes) => ( Some(bytes), 200 ),
				Err (err) => { return ( None, err.raw_os_error().unwrap() ); }
			} } // end return match
	} // end impl CrossFetchPre

}
