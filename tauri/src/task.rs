use maa_framework::{
    diff_task::{DiffTaskBuilder, List},
    instance::TaskParam,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

use crate::{
    config::{combat::CombatConfig, drive_combat::DriveCombatConfig, travel::TravelConfig},
    MaaZError,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TaskRunningState {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskStatus {
    pub id: Option<i64>,
    #[serde(rename = "taskType")]
    pub task_type: TaskType,
    pub state: TaskRunningState,
}

impl From<TaskType> for TaskStatus {
    fn from(task_type: TaskType) -> Self {
        Self {
            id: None,
            task_type,
            state: TaskRunningState::Pending,
        }
    }
}

macro_rules! task_type {
    ($($variant:ident),+) => {
        #[derive(Serialize, Deserialize, Clone, Copy, Debug)]
        pub enum TaskType {
            $($variant),+
        }

        impl TaskType {
            pub fn get_string(self) -> String {
                match self {
                    $(TaskType::$variant => stringify!($variant).to_owned()),+
                }
            }
        }

        impl TryFrom<String> for TaskType {
            type Error = MaaZError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str() {
                    $(stringify!($variant) => Ok(TaskType::$variant),)+
                    _ => Err(MaaZError::UnknowTaskError(value)),
                }
            }
        }
    };
}

task_type!(StartUp, Combat, DriveCombat, Travel);

pub struct CombatParam {
    pub times: u32,
}

impl From<CombatConfig> for CombatParam {
    fn from(config: CombatConfig) -> Self {
        Self {
            times: config.times,
        }
    }
}

impl Serialize for CombatParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let times = if self.times == 0 {
            u32::MAX
        } else {
            self.times
        };
        let task = DiffTaskBuilder::default()
            .times_limit(Some(times))
            .build()
            .ok();
        json!({
            "GoToCombat":task
        })
        .serialize(serializer)
    }
}

impl TaskParam for CombatParam {}

pub struct DriveCombatParam {
    pub use_fuel: bool,
}

impl From<DriveCombatConfig> for DriveCombatParam {
    fn from(config: DriveCombatConfig) -> Self {
        Self {
            use_fuel: config.use_fuel,
        }
    }
}

impl Serialize for DriveCombatParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let task = DiffTaskBuilder::default()
            .enabled(Some(self.use_fuel))
            .build()
            .ok();
        json!({
            "UseBullet":task
        })
        .serialize(serializer)
    }
}

impl TaskParam for DriveCombatParam {}

pub struct TravelParam {
    pub destination: String,
}

impl From<TravelConfig> for TravelParam {
    fn from(config: TravelConfig) -> Self {
        Self {
            destination: config.destination,
        }
    }
}

impl Serialize for TravelParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let task = DiffTaskBuilder::default()
            .text(Some(List::Single(self.destination.clone())))
            .build()
            .ok();
        json!({
            "Recognition":task
        })
        .serialize(serializer)
    }
}

impl TaskParam for TravelParam {}
