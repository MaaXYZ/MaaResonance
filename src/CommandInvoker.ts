import { invoke } from "@tauri-apps/api/core";
import DeviceInfo from "./interface/DeviceInfo";
import TaskStatus, { TaskType } from "./interface/TaskStatus";
import { MaaConfig } from "./interface/MaaConfig";
import { MaaAdbControllerType } from "./interface/AppConfig";

export default class CommandInvoker {
    public static async initMaa(): Promise<void> {
        return invoke("init_maa");
    }

    public static async findDevices(): Promise<DeviceInfo[]> {
        return invoke("find_devices");
    }

    public static async connectTo(device: DeviceInfo): Promise<void> {
        return invoke("connect_to_device", { device: device });
    }

    public static async startUpTask(): Promise<void> {
        return invoke("start_up");
    }

    public static async removeFromQueue(index: number): Promise<void> {
        return invoke("remove_from_queue", {
            index: index,
        });
    }

    public static async addTaskToQueue(
        task: TaskType,
        appendNext: boolean = false
    ): Promise<void> {
        return invoke("add_task_to_queue", {
            task: task,
            appendNext: appendNext,
        });
    }

    public static async getQueue(): Promise<TaskStatus[]> {
        return invoke("get_queue");
    }

    public static async startQueue(): Promise<void> {
        return invoke("start_queue");
    }

    public static async stopQueue(): Promise<void> {
        return invoke("stop_queue");
    }

    public static async getQueueState(): Promise<boolean> {
        return invoke("get_queue_state");
    }

    public static async getConfig(): Promise<MaaConfig> {
        return invoke("get_config");
    }

    public static async setClientType(clientType: string): Promise<void> {
        return invoke("set_client_type", {
            value: clientType,
        });
    }

    public static async setControllerType(
        controllerType: MaaAdbControllerType
    ): Promise<void> {
        return invoke("set_controller_type", {
            value: controllerType,
        });
    }

    public static async startMiniWindow(): Promise<void> {
        return invoke("start_mini_window");
    }

    public static async closeWindow(): Promise<void> {
        return invoke("close_window");
    }

    public static async minimizeWindow(): Promise<void> {
        return invoke("minimize_window");
    }

    public static async openSettings(): Promise<void> {
        return invoke("open_settings_window");
    }
}
