// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(error_generic_member_access)]
#![allow(clippy::result_large_err)]

use std::backtrace::Backtrace;
use std::error::request_ref;
use std::process::exit;
use std::sync::Arc;

use config::ConfigHolder;
use maa_framework::{
    controller::MaaControllerInstance, instance::MaaInstance, resource::MaaResourceInstance,
    toolkit::MaaToolkit,
};
use queue::TaskQueue;
use serde::Serialize;
use tauri::{
    async_runtime::{channel, spawn, Mutex},
    App, Manager,
};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::MessageDialogBuilder;
use thiserror::Error;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::callback::{setup_callback, CallbackEventHandler};

mod callback;
mod commands;
mod config;
mod queue;
mod task;

pub type ConfigHolderState = Arc<Mutex<config::ConfigHolder>>;

pub type TaskQueueState = Arc<Mutex<TaskQueue>>;

pub type Instance = MaaInstance<CallbackEventHandler>;
pub type ResourceInstance = MaaResourceInstance<CallbackEventHandler>;
pub type ControllerInstance = Mutex<Option<MaaControllerInstance<CallbackEventHandler>>>;

pub fn run() {
    #[cfg(feature = "mock")]
    println!("Running in MOCK mode");

    let _guard = init_tracing();

    tracing::info!("Starting Maa");

    let ret = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let ret = setup_app(app);

            if let Err(e) = ret {
                let dialog = app.dialog();
                let backtrace = request_ref::<Backtrace>(&e);
                let backtrace = backtrace.map(|b| format!("{b}")).unwrap_or_default();
                let msg = format!("Error during initialization: {e}\n{backtrace}");
                MessageDialogBuilder::new(dialog.clone(), "Error during initialization", msg)
                    .kind(tauri_plugin_dialog::MessageDialogKind::Error)
                    .blocking_show();

                exit(1);
            }

            #[cfg(debug_assertions)]
            app.get_webview_window("main")
                .ok_or(MaaZError::WindowNotFoundError("main".to_owned()))?
                .open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::device::find_devices,
            commands::device::connect_to_device,
            commands::config::get_config,
            commands::config::set_client_type,
            commands::config::set_controller_type,
            commands::task::add_task_to_queue,
            commands::task::start_queue,
            commands::task::stop_queue,
            commands::task::remove_from_queue,
            commands::task::get_queue,
            commands::task::get_queue_state,
            commands::window::close_window,
            commands::window::minimize_window,
            commands::window::maximize_window,
            commands::init_maa,
            commands::start_mini_window,
            commands::set_window_on_top,
            commands::open_settings_window
        ])
        .run(tauri::generate_context!());

    if let Err(e) = ret {
        tracing::error!("Error during tauri run: {:?}", e);
    }
}

fn setup_app(app: &mut App) -> MaaZInnerResult<()> {
    let path = std::env::current_exe()?;

    let config_file = path.with_file_name("maa.toml");
    let config = ConfigHolder::new(config_file)?;
    let config = Arc::new(Mutex::new(config));
    app.manage(Arc::clone(&config));

    let task_queue = TaskQueueState::default();
    app.manage(Arc::clone(&task_queue));

    let (sender, receiver) = channel(5);
    let callback = CallbackEventHandler::new(sender);
    let instance = MaaInstance::new(Some(callback));
    let instance = Arc::new(instance);
    app.manage(Arc::clone(&instance));

    let toolkit = MaaToolkit::new()?;
    app.manage(toolkit);

    let controller = ControllerInstance::default();
    app.manage(Arc::new(controller));

    let resource: MaaResourceInstance<CallbackEventHandler> = MaaResourceInstance::new(None);
    app.manage(resource);

    let app_handle = app.app_handle().clone();

    spawn(async move {
        setup_callback(
            app_handle,
            Arc::clone(&task_queue),
            config,
            Arc::clone(&instance),
            receiver,
        )
        .await;
    });

    Ok(())
}

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "maa.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    #[allow(clippy::expect_used)]
    let timer = OffsetTime::local_rfc_3339().expect("error while creating timer");

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .with_timer(timer)
        .init();

    guard
}

pub type MaaZInnerResult<T> = Result<T, MaaZInnerError>;

#[derive(Error, Debug)]
pub enum MaaZInnerError {
    #[error("IO Error: {0}")]
    IOError(
        #[from] std::io::Error,
        #[backtrace] std::backtrace::Backtrace,
    ),

    #[error("TOML De Error: {0}")]
    TOMLDeError(
        #[from] toml::de::Error,
        #[backtrace] std::backtrace::Backtrace,
    ),

    #[error("TOML Ser Error: {0}")]
    TOMLSerError(
        #[from] toml::ser::Error,
        #[backtrace] std::backtrace::Backtrace,
    ),

    #[error("Maa Error: {0}")]
    MaaError(
        #[from] maa_framework::error::Error,
        #[backtrace] std::backtrace::Backtrace,
    ),
}

pub type MaaZResult<T> = Result<T, MaaZError>;

#[derive(Serialize, Debug, Error)]
pub enum MaaZError {
    #[error("UTF8 Error: {0}")]
    Utf8Error(String),
    #[error("IO Error: {0}")]
    IOError(String),
    #[error("Tauri Error: {0}")]
    TauriError(String),
    #[error("Maa Error: {0}")]
    MaaError(String),
    #[error("Resource Init Error")]
    ResourceInitError,
    #[error("Connection Error")]
    ConnectionError,
    #[error("Task Queue Did not Start")]
    QueueDidnotStart,
    #[error("Unknow Task Error: {0}")]
    UnknowTaskError(String),
    #[error("MaaZ Inner Error: {0}")]
    MaaZInnerError(String),
    #[error("Window Not Found Error: {0}")]
    WindowNotFoundError(String),
}

impl From<std::str::Utf8Error> for MaaZError {
    fn from(e: std::str::Utf8Error) -> Self {
        MaaZError::Utf8Error(e.to_string())
    }
}

impl From<std::io::Error> for MaaZError {
    fn from(e: std::io::Error) -> Self {
        MaaZError::IOError(e.to_string())
    }
}

impl From<tauri::Error> for MaaZError {
    fn from(e: tauri::Error) -> Self {
        MaaZError::TauriError(e.to_string())
    }
}

impl From<maa_framework::error::Error> for MaaZError {
    fn from(e: maa_framework::error::Error) -> Self {
        MaaZError::MaaError(e.to_string())
    }
}

impl From<MaaZInnerError> for MaaZError {
    fn from(e: MaaZInnerError) -> Self {
        MaaZError::MaaZInnerError(e.to_string())
    }
}
