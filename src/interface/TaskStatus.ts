export const allTaskTypes = ["StartUp","Award"] as const;

export type TaskType = (typeof allTaskTypes)[number];

export type TaskState = "Pending" | "Running" | "Completed" | "Failed";

export default interface TaskStatus {
    id?: number;
    taskType: TaskType;
    state: TaskState;
}
