use std::sync::Arc;

use maa_framework::{instance::MaaInstance, msg::MaaMsg, CallbackHandler};
use tauri::{
    async_runtime::{self, spawn, Receiver, Sender},
    AppHandle, Manager,
};
use tauri_plugin_notification::NotificationExt;
use tracing::{error, info, trace, trace_span};

use crate::TaskQueueState;

pub const CALLBACK_EVENT: &str = "callback";
pub const QUEUE_DONE_EVENT: &str = "queue-done";

macro_rules! notify {
    ($app:expr,$title:expr) => {
        match $app.notification().builder().title($title).show() {
            Ok(_) => {}
            Err(e) => {
                error!("Error while showing notification: {}", e);
            }
        }
    };

    ($app:expr,$title:expr,$body:expr) => {
        match $app
            .notification()
            .builder()
            .title($title)
            .body($body)
            .show()
        {
            Ok(_) => {}
            Err(e) => {
                error!("Error while showing notification: {}", e);
            }
        }
    };
}

pub struct CallbackEventHandler {
    sender: Sender<MaaMsg>,
}

impl CallbackHandler for CallbackEventHandler {
    fn handle(&mut self, msg: MaaMsg) {
        let sender = self.sender.clone();
        spawn(async move {
            #[allow(clippy::unwrap_used)]
            sender.send(msg).await.unwrap();
        });
    }
}

impl CallbackEventHandler {
    pub fn new(sender: Sender<MaaMsg>) -> Self {
        Self { sender }
    }
}

pub async fn setup_callback(
    app: AppHandle,
    queue: TaskQueueState,
    instance: Arc<MaaInstance<CallbackEventHandler>>,
    mut recv: Receiver<MaaMsg>,
) {
    while let Some(msg) = recv.recv().await {
        let span = trace_span!("callback");
        let _guard = span.enter();

        let app_handle = app.clone();
        let queue = Arc::clone(&queue);

        let instance = Arc::clone(&instance);

        #[allow(clippy::unwrap_used)]
        app.emit(CALLBACK_EVENT, ()).unwrap();

        #[allow(clippy::match_same_arms)]
        match msg {
            MaaMsg::TaskCompleted(_task) => {
                let queue = Arc::clone(&queue);
                async_runtime::spawn(async move {
                    info!("Running next task");
                    let mut queue = queue.lock().await;
                    let has_next = queue.run_next(&instance, true);
                    if !has_next {
                        #[allow(clippy::unwrap_used)]
                        app_handle.emit(QUEUE_DONE_EVENT, ()).unwrap();
                        notify!(app_handle, "Task Queue Finished");
                    }
                });
            }
            MaaMsg::TaskFailed(task) => {
                error!("Task failed: {:?}", task);
                notify!(app_handle, "Task Failed", &task.name);
                let mut queue = queue.lock().await;
                let has_next = queue.run_next(&instance, false);
                if !has_next {
                    #[allow(clippy::unwrap_used)]
                    app_handle.emit(QUEUE_DONE_EVENT, ()).unwrap();
                    notify!(app_handle, "Task Queue Finished");
                }
            }
            _ => {
                trace!("Unhandled message: {:?}", msg);
            }
        }
    }
}
