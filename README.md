
# CrossFetch
CrossFetch is a library for fetching files on both native and WebAssembly platforms.
Files can be relative local paths or http/https urls, both work on either platform.
Files are loaded into an array of bytes. Use the .to_string() or .as_str() method on
a Vec&lt;u8&gt; to convert it into a unicode string.

## Roadmap:
- [x] Fetch local files as bytes (inc. WASM)
- [x] Fetch remote http and https files as bytes
- [x] Easy conversion to strings

