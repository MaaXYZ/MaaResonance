use std::sync::Arc;

use tauri::State;
use tracing::info;

use crate::{
    queue::QueueStartStatus,
    task::{TaskStatus, TaskType},
    ConfigHolderState, Instance, MaaZError, MaaZResult, TaskQueueState,
};

#[tauri::command]
pub async fn add_task_to_queue(
    task_queue: State<'_, TaskQueueState>,
    task: String,
    append_next: Option<bool>,
) -> MaaZResult<()> {
    info!("Adding task {} to queue", task);
    let append_next = append_next.unwrap_or(false);
    let mut queue = task_queue.lock().await;
    let task_type = TaskType::try_from(task)?;
    if append_next {
        queue.append_next(task_type);
    } else {
        queue.push(task_type);
    }
    Ok(())
}

#[tauri::command]
pub async fn start_queue(
    task_queue: State<'_, TaskQueueState>,
    inst: State<'_, Arc<Instance>>,
    config: State<'_, ConfigHolderState>,
) -> MaaZResult<()> {
    tracing::info!("Starting task queue");
    let mut queue = task_queue.lock().await;
    let config = config.clone();
    let config = config.lock().await;
    let ret = queue.start(&inst, config.config());
    info!("Queue start status: {:?}", ret);
    if matches!(ret, QueueStartStatus::Started) {
        Ok(())
    } else {
        Err(MaaZError::QueueDidnotStart)
    }
}

#[tauri::command]
pub async fn stop_queue(
    task_queue: State<'_, TaskQueueState>,
    handle: State<'_, Arc<Instance>>,
) -> MaaZResult<()> {
    tracing::info!("Stopping task queue");
    let mut queue = task_queue.lock().await;
    queue.stop(&handle);
    Ok(())
}

#[tauri::command]
pub async fn remove_from_queue(
    task_queue: State<'_, TaskQueueState>,
    index: usize,
) -> MaaZResult<()> {
    tracing::info!("Removing task from queue");
    let mut queue = task_queue.lock().await;
    queue.remove(index);
    Ok(())
}

#[tauri::command]
pub async fn get_queue(task_queue: State<'_, TaskQueueState>) -> MaaZResult<Vec<TaskStatus>> {
    let queue = task_queue.lock().await;
    Ok(queue.current_queue())
}

#[tauri::command]
pub async fn get_queue_state(task_queue: State<'_, TaskQueueState>) -> MaaZResult<bool> {
    let queue = task_queue.lock().await;
    Ok(!queue.idle())
}
