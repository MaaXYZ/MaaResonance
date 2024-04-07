import { defineStore } from "pinia";
import { useDeviceStateStore } from "./DeviceStateStore";
import { MaaConfig } from "@/interface/MaaConfig";
import CommandInvoker from "@/CommandInvoker";
import {
    MaaAdbControllerScreencapType,
    MaaAdbControllerKeyType,
    MaaAdbControllerTouchType,
} from "@/interface/AppConfig";

export const useMaaStateStore = defineStore("maa-state", {
    state: () => {
        return {
            resourceInited: false,
            config: null as MaaConfig | null,

            miniWindowOpened: false,
        };
    },
    getters: {
        isMaaReady(state) {
            const deviceStateStore = useDeviceStateStore();
            return state.resourceInited && deviceStateStore.isDeviceConnected;
        },
    },
    actions: {
        noteResourceInited() {
            this.resourceInited = true;
        },
        async getConfig() {
            CommandInvoker.getConfig().then((config) => {
                this.config = config;
            });
        },
        async setConfig(configName: string, value: any) {
            CommandInvoker.setConfig(configName, value).then(() => {
                this.getConfig();
            });
        },
        async setControllerTouchType(touchType: MaaAdbControllerTouchType) {
            if (this.config) {
                const updatedConfig = {
                    ...this.config.appConfig.adb_controller_type,
                    touch_type: touchType,
                };
                CommandInvoker.setControllerType(updatedConfig).then(() => {
                    this.getConfig();
                });
            }
        },
        async setControllerKeyType(keyType: MaaAdbControllerKeyType) {
            if (this.config) {
                const updatedConfig = {
                    ...this.config.appConfig.adb_controller_type,
                    key_type: keyType,
                };
                CommandInvoker.setControllerType(updatedConfig).then(() => {
                    this.getConfig();
                });
            }
        },
        async setControllerScreencapType(
            screencapType: MaaAdbControllerScreencapType
        ) {
            if (this.config) {
                const updatedConfig = {
                    ...this.config.appConfig.adb_controller_type,
                    screencap_type: screencapType,
                };
                CommandInvoker.setControllerType(updatedConfig).then(() => {
                    this.getConfig();
                });
            }
        },
    },
});
