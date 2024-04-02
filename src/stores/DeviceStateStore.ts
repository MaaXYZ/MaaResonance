import CommandInvoker from "@/CommandInvoker";
import DeviceInfo from "@/interface/DeviceInfo";
import { defineStore } from "pinia";

export const useDeviceStateStore = defineStore('device-state',{
    state: ()=>{
        return {
            devices: [] as DeviceInfo[],
            connectedDevice: null as DeviceInfo | null
        }
    },
    getters: {
        isDeviceConnected: (state) => {
            return state.connectedDevice !== null;
        }
    },
    actions: {
        async loadDevices() {
            const devices = await CommandInvoker.findDevices()
            this.devices = devices;
        },
        async connectTo(device:DeviceInfo) {
            return CommandInvoker.connectTo(device).then(() => {
                console.log("Connected to device", device);
                this.connectedDevice = device;
            });
        }
    }
})