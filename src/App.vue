<script setup lang="ts">
import DeviceConnection from "./views/DeviceConnection.vue";
import TaskDispatch from "./views/TaskDispatch.vue";
import { onMounted, ref } from "vue";
import CommandInvoker from "./CommandInvoker";
import { useMaaStateStore } from "./stores/MaaStateStore";
import CloseIcon from "./assets/icons/CloseIcon.vue";
import SettingsIcon from "./assets/icons/SettingsIcon.vue";
import MinimizeIcon from "./assets/icons/MinimizeIcon.vue";
import { getTheme } from "mdui/functions/getTheme";
import { setTheme } from "mdui/functions/setTheme";
import LightIcon from "./assets/icons/LightIcon.vue";
import DarkIcon from "./assets/icons/DarkIcon.vue";

const maaStateStore = useMaaStateStore();

onMounted(() => {
    CommandInvoker.initMaa().then(() => {
        console.log("Maa initialized");
        maaStateStore.noteResourceInited();
    });
});

const theme = ref(getTheme());

function closeWindow() {
    CommandInvoker.closeWindow();
}

function minimizeWindow() {
    CommandInvoker.minimizeWindow();
}

function openSettings() {
    CommandInvoker.openSettings();
}

function toggleTheme() {
    if (theme.value === "light") {
        setTheme("dark");
        theme.value = "dark";
    } else {
        setTheme("light");
        theme.value = "light";
    }
}

</script>

<template>
    <div class="h-screen w-full flex flex-col select-none">
        <div
            data-tauri-drag-region
            class="app-heade h-10 select-none flex flex-row items-center justify-between p-2"
        >
            <div class="header">
                <span class="text-lg text-center font-bold">{{
                    $t("appName")
                }}</span>
            </div>
            <div class="controls">
                <mdui-button-icon @click="openSettings">
                    <mdui-icon>
                        <SettingsIcon />
                    </mdui-icon>
                </mdui-button-icon>
                <mdui-button-icon @click="toggleTheme">
                    <mdui-icon>
                        <mdui-icon v-if="theme === 'light'">
                            <LightIcon />
                        </mdui-icon>
                        <mdui-icon v-else>
                            <DarkIcon />
                        </mdui-icon>
                    </mdui-icon>
                </mdui-button-icon>
                <mdui-button-icon @click="minimizeWindow">
                    <mdui-icon>
                        <MinimizeIcon />
                    </mdui-icon>
                </mdui-button-icon>
                <mdui-button-icon @click="closeWindow">
                    <mdui-icon>
                        <CloseIcon />
                    </mdui-icon>
                </mdui-button-icon>
            </div>
        </div>
        <div class="flex flex-row flex-grow">
            <device-connection class="conn" />
            <task-dispatch class="taskd" />
        </div>
    </div>
</template>

<style scoped>
.conn {
    width: 30%;
}

.taskd {
    width: 70%;
}

.app_header {
    background-color: var(--md-sys-color-surface);
}
</style>
