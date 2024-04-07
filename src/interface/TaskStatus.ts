export const allTaskTypes = ["StartUp", "Combat", "DriveCombat"] as const;

export type TaskType = (typeof allTaskTypes)[number];

export const allTaskTypesContent: Record<TaskType, string> = {
    StartUp: "tasks.startUp",
    Combat: "tasks.combat",
    DriveCombat: "tasks.driveCombat",
};

export type TaskState = "Pending" | "Running" | "Completed" | "Failed";

export default interface TaskStatus {
    id?: number;
    taskType: TaskType;
    state: TaskState;
}
