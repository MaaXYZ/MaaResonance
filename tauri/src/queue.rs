use maa_framework::instance::MaaInstance;
use serde::Serialize;
use serde_json::{json, Value};
use tracing::{error, info, trace, trace_span};

use crate::{
    callback::CallbackEventHandler,
    config::Config,
    task::{CombatParam, TaskRunningState, TaskStatus, TaskType, TravelParam},
    MaaZResult,
};

#[derive(Default, Serialize)]
pub struct TaskQueue {
    queue: Vec<(TaskStatus, Value)>,
}

#[derive(Debug)]
pub enum QueueStartStatus {
    Started,
    AlreadyRunning,
    NoPendingTasks,
}

impl TaskQueue {
    pub fn current_queue(&self) -> Vec<TaskStatus> {
        self.queue
            .iter()
            .map(|(status, _)| status.clone())
            .collect()
    }

    pub fn push(&mut self, task: TaskType, config: Config) -> MaaZResult<()> {
        let param = match task {
            TaskType::StartUp => json!({}),
            TaskType::Combat => serde_json::to_value(CombatParam::from(config.combat))?,
            TaskType::Travel => serde_json::to_value(TravelParam::from(config.travel))?,
        };
        trace!("Pushing task {:?} with param {:?}", task, param);
        self.queue.push((task.into(), param));
        Ok(())
    }

    /// Remove a task from the queue
    pub fn remove(&mut self, index: usize) {
        self.queue.remove(index);
    }

    /// Mark the running task as completed
    pub fn complete_running(&mut self, success: bool) {
        if let Some(index) = self
            .queue
            .iter()
            .position(|(t, _)| matches!(t.state, TaskRunningState::Running))
        {
            self.queue[index].0.state = if success {
                TaskRunningState::Completed
            } else {
                TaskRunningState::Failed
            };
            trace!("Task completed: {:?} with {success}", self.queue[index]);
        }
    }

    /// Mark the running task as completed and start the next task
    pub fn run_next(&mut self, handle: &MaaInstance<CallbackEventHandler>, success: bool) -> bool {
        let span = trace_span!("run_next");
        let _guard = span.enter();
        self.complete_running(success);
        trace!("Running next task");
        if let Some(index) = self
            .queue
            .iter()
            .position(|(t, _)| matches!(t.state, TaskRunningState::Pending))
        {
            self.queue[index].0.state = TaskRunningState::Running;
            let (task, param) = &mut self.queue[index];
            info!("Running task {:?}", task);

            let entry = task.task_type.get_string();

            let id = handle.post_task(&entry, param.clone());
            task.id = Some(id);
            true
        } else {
            info!("No more tasks to run");
            false
        }
    }

    pub fn idle(&self) -> bool {
        !self
            .queue
            .iter()
            .any(|(t, _)| matches!(t.state, TaskRunningState::Running))
    }

    pub fn start(&mut self, handle: &MaaInstance<CallbackEventHandler>) -> QueueStartStatus {
        #[cfg(feature = "mock")]
        {
            return QueueStartStatus::Started;
        }

        if !self.idle() {
            info!("Task queue is already running");
            return QueueStartStatus::AlreadyRunning;
        }

        let has_pending = self
            .queue
            .iter()
            .any(|(t, _)| matches!(t.state, TaskRunningState::Pending));
        if !has_pending {
            info!("No pending tasks to run");
            return QueueStartStatus::NoPendingTasks;
        }

        self.run_next(handle, true);
        QueueStartStatus::Started
    }

    /// This sends a stop signal to fw and mark the running task as Pending
    pub fn stop(handle: &MaaInstance<CallbackEventHandler>) {
        let stop_ret = handle.post_stop();
        if stop_ret.is_err() {
            error!("Error while stopping task");
        }
    }
}
