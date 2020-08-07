#[cfg(not(target_arch = "wasm32"))]
compile_error!("This libary is only intended to be built for wasm32-unknown-unknown and won't work otherwise.");
