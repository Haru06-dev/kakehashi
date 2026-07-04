// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Redirect WebView2's user data folder (cache, IndexedDB, the AI model
    // cache, everything). Checks KAKEHASHI_DATA_DIR first — an environment
    // variable only set on the developer's own machine, pointing at H: to
    // avoid C: drive usage there. Anyone else running this app (who won't
    // have that variable set) falls back to a folder next to the .exe
    // itself, which is fully portable regardless of their drive letters
    // or install location.
    let data_dir = std::env::var("KAKEHASHI_DATA_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("webview2data")))
                .unwrap_or_else(|| std::path::PathBuf::from("webview2data"))
        });

    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", data_dir);
    app_lib::run();
}