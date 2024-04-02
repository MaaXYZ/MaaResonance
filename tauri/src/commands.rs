#![allow(clippy::needless_pass_by_value)]
// Strange since this seems to be fixed already. https://github.com/rust-lang/rust-clippy/issues/4980
#![allow(clippy::let_underscore_must_use)]
#![allow(clippy::used_underscore_binding)]

use std::sync::Arc;

use maa_framework::MaaStatus;
use tauri::{AppHandle, State, Window};
use tracing::{trace, trace_span};

use crate::{Instance, MaaZResult, ResourceInstance};
pub mod config;
pub mod device;
pub mod task;
pub mod window;

#[cfg(feature = "mock")]
mod mock;

#[tauri::command]
pub async fn init_maa(
    inst: State<'_, Arc<Instance>>,
    resource: State<'_, ResourceInstance>,
) -> MaaZResult<()> {
    let span = trace_span!("init_maa");
    let _guard = span.enter();

    #[cfg(feature = "mock")]
    return Ok(());

    // init resources
    trace!("init resources");
    let resource_path = "resources";
    let id = resource.post_path(resource_path);

    let ret = resource.wait(id)?;

    if let MaaStatus::Success = ret {
        inst.bind_resource(&resource)?;
        return Ok(());
    }

    Err(crate::MaaZError::ResourceInitError)
}

#[tauri::command]
#[allow(clippy::unused_async)]
// This needs to be async otherwise it will cause deadlock on windows
pub async fn start_mini_window(app: AppHandle) -> MaaZResult<()> {
    tauri::WebviewWindowBuilder::new(&app, "mini", tauri::WebviewUrl::App("mini.html".into()))
        .title("MaaZ Mini")
        .inner_size(256.0, 512.0)
        .decorations(false)
        .maximizable(false)
        .build()?
        .show()?;

    Ok(())
}

#[tauri::command]
pub async fn set_window_on_top(window: Window, on_top: bool) -> MaaZResult<()> {
    window.set_always_on_top(on_top)?;
    Ok(())
}

#[tauri::command]
pub async fn open_settings_window(app: AppHandle) -> MaaZResult<()> {
    tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("settings.html".into()),
    )
    .title("MaaZ Settings")
    .inner_size(800.0, 600.0)
    .build()?
    .show()?;

    Ok(())
}
