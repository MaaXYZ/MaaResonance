import { listen } from "@tauri-apps/api/event";
import { useTaskQueueStore } from "./stores/TaskQueueStore";
import CallbackPayload from "./interface/CallbackPayload";
import { useMaaStateStore } from "./stores/MaaStateStore";

export const setupListener = () => {

    const taskQueueStore = useTaskQueueStore();

    const maaStateStore = useMaaStateStore();

    listen<CallbackPayload>("callback", (event) => {
        console.log("Callback received: ", event.payload);
        taskQueueStore.updateQueue();
    });

    listen("queue-done", (_event) => {
        taskQueueStore.queueRunning = false;
    });

    listen("config-changed", (_event) => {
        maaStateStore.getConfig();
    });
};
