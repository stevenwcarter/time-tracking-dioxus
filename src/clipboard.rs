use dioxus::logger::tracing::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[wasm_bindgen]
pub async fn copy_to_clipboard(text: &str) {
    if let Some(window) = window() {
        let clipboard = window.navigator().clipboard();

        match JsFuture::from(clipboard.write_text(text)).await {
            Ok(_) => {
                debug!("Text copied to clipboard: {}", text);
            }
            Err(e) => {
                error!("Failed to copy text to clipboard: {:?}", e);
            }
        }
    }
}
