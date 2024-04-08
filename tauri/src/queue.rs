use maa_framework::instance::MaaInstance;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, trace, trace_span};

use crate::{
    callback::CallbackEventHandler,
    config::Config,
    task::{CombatParam, DriveCombatParam, TaskRunningState, TaskStatus, TaskType},
};

#[derive(Default, Serialize, Deserialize)]
pub struct TaskQueue {
    queue: Vec<TaskStatus>,
}

#[derive(Debug)]
pub enum QueueStartStatus {
    Started,
    AlreadyRunning,
    NoPendingTasks,
}

impl TaskQueue {
    pub fn current_queue(&self) -> Vec<TaskStatus> {
        self.queue.clone()
    }

    pub fn push(&mut self, task: TaskType) {
        self.queue.push(task.into());
    }

    /// Append a task to right after the running task
    pub fn append_next(&mut self, task: TaskType) {
        if let Some(index) = self
            .queue
            .iter()
            .position(|t| matches!(t.state, TaskRunningState::Pending))
        {
            self.queue.insert(index, task.into());
        } else {
            self.queue.push(task.into());
        }
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
            .position(|t| matches!(t.state, TaskRunningState::Running))
        {
            self.queue[index].state = if success {
                TaskRunningState::Completed
            } else {
                TaskRunningState::Failed
            };
            trace!("Task completed: {:?} with {success}", self.queue[index]);
        }
    }

    /// Mark the running task as completed and start the next task
    pub fn run_next(
        &mut self,
        handle: &MaaInstance<CallbackEventHandler>,
        config: Config,
        success: bool,
    ) -> bool {
        let span = trace_span!("run_next");
        let _guard = span.enter();
        self.complete_running(success);
        trace!("Running next task");
        if let Some(index) = self
            .queue
            .iter()
            .position(|t| matches!(t.state, TaskRunningState::Pending))
        {
            self.queue[index].state = TaskRunningState::Running;
            let task = &mut self.queue[index];
            info!("Running task {:?}", task);

            let entry = task.task_type.get_string();

            let id = match task.task_type {
                TaskType::StartUp => handle.post_task(&entry, json!({})),
                TaskType::Combat => {
                    let config = config.combat;
                    let param: CombatParam = config.into();
                    handle.post_task(&entry, param)
                }
                TaskType::DriveCombat => {
                    let config = config.drive_combat;
                    let param: DriveCombatParam = config.into();
                    handle.post_task(&entry, param)
                }
            };
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
            .any(|t| matches!(t.state, TaskRunningState::Running))
    }

    pub fn start(
        &mut self,
        handle: &MaaInstance<CallbackEventHandler>,
        config: Config,
    ) -> QueueStartStatus {
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
            .any(|t| matches!(t.state, TaskRunningState::Pending));
        if !has_pending {
            info!("No pending tasks to run");
            return QueueStartStatus::NoPendingTasks;
        }

        self.run_next(handle, config, true);
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
