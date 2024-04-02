<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import DeviceItem from "@/components/DeviceItem.vue";
import { useDeviceStateStore } from "@/stores/DeviceStateStore";
import 'mdui/components/circular-progress'
import 'mdui/components/button'
import 'mdui/components/list'
import 'mdui/components/list-item'

const loadingDevices = ref(true);

const deviceStateStore = useDeviceStateStore();

const connectedDeviceTitleClass = computed(() => {
    return {
        title_secondary: deviceStateStore.isDeviceConnected,
        title_primary: !deviceStateStore.isDeviceConnected,
    };
});

onMounted(() => {
    loadDevices();
});

function loadDevices() {
    loadingDevices.value = true;
    deviceStateStore.loadDevices().finally(() => {
        loadingDevices.value = false;
    });
}
</script>

<template>
    <div class="flex flex-col h-full container">
        <div class="flex justify-center">
            <p :class="connectedDeviceTitleClass">Connected Device</p>
        </div>

        <div class="flex flex-col" v-if="!deviceStateStore.isDeviceConnected">
            <p class="text_small">No device connected</p>
        </div>

        <device-item v-else :device="deviceStateStore.connectedDevice!" />

        <div
            class="title_secondary flex flex-row items-center align-middle justify-center"
            v-if="loadingDevices"
        >
            <mdui-circular-progress></mdui-circular-progress>Searching
            for devices...
        </div>
        <div
            class="flex flex-col"
            v-else-if="deviceStateStore.devices.length == 0"
        >
            <p class="title_secondary">No devices found</p>
            <mdui-button @click="loadDevices" strong variant="filled" class="mx-4">
                Rescan Devices
            </mdui-button>
        </div>
        <div class="flex flex-col justify-center items-center" v-else>
            <p>Available Devices</p>
            <mdui-list class="device_list">
                <mdui-list-item
                    v-for="device in deviceStateStore.devices"
                    :key="device.name"
                >
                    <device-item :device="device" />
                </mdui-list-item>
            </mdui-list>
        </div>
    </div>
</template>

<style scoped>
.device_list {
    width: 95%;
}

.title_secondary {
    font-size: 1.5rem;
    color: #a0aec0;
    text-align: center;
    margin-top: 2rem;
}

.title_primary {
    font-size: 1.5rem;
    color: #2d3748;
    text-align: center;
    margin-top: 2rem;
}

.text_small {
    font-size: 1rem;
    color: #a0aec0;
    text-align: center;
}
</style>
