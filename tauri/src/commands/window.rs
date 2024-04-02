use tauri::WebviewWindow;

use crate::MaaZResult;

#[tauri::command]
pub fn close_window(window: WebviewWindow) -> MaaZResult<()> {
    window.close()?;
    Ok(())
}

#[tauri::command]
pub fn minimize_window(window: WebviewWindow) -> MaaZResult<()> {
    window.minimize()?;
    Ok(())
}

#[tauri::command]
pub fn maximize_window(window: WebviewWindow) -> MaaZResult<()> {
    window.maximize()?;
    Ok(())
}
