// based on https://users.rust-lang.org/t/rust-wasm-async-sleeping-for-100-milli-seconds-goes-up-to-1-minute/81177
use crate::Error;

/// Sleep asynchronously for the given number of milliseconds on WASM targets.
///
/// Uses `tokio_with_wasm` rather than `web_sys`'s `window().setTimeout` so that
/// it also works in non-browser WASM environments (e.g. Node.js), which have no
/// `window` object.
#[cfg(target_arch = "wasm32")]
pub async fn async_sleep(millis: u64) -> Result<(), Error> {
    use tokio_with_wasm::alias as tokio;
    tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
/// Sleep asynchronously for the given number of milliseconds on non-WASM targets.
pub async fn async_sleep(millis: u64) -> Result<(), Error> {
    tokio::time::sleep(tokio::time::Duration::from_millis(millis)).await;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
/// Get the current time in milliseconds since the UNIX epoch
pub async fn async_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_millis() as u64
}

#[cfg(target_arch = "wasm32")]
/// Get the current time in milliseconds since the UNIX epoch
pub async fn async_now() -> u64 {
    js_sys::Date::now() as u64
}
