use tauri::State;
use tokio::time::Duration;

use crate::model::StreamState;
use crate::window_info::macos_window_info::stream_macos;

pub async fn stream_title<'r>(app: tauri::AppHandle, stream_state: State<'r, StreamState>) {
    let cancel_flag = stream_state.cancel_flag.clone();
    *cancel_flag.lock().await = false;

    let sleep_duration = Duration::from_millis(300);

    // Start macOS window tracking
    let _ = stream_macos(cancel_flag, app, sleep_duration).map_err(|e| eprintln!("{}", e));
}

pub async fn stop_stream(stream_state: State<'_, StreamState>) {
    let mut cancel_flag = stream_state.cancel_flag.lock().await;
    *cancel_flag = true;
}
