<script setup lang="ts">
import { useTaskQueueStore } from "@/stores/TaskQueueStore";
import { onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import CommandInvoker from "@/CommandInvoker";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import "mdui/components/list";
import "mdui/components/list-item";
import "mdui/components/circular-progress";
import "mdui/components/icon";
import "mdui/components/checkbox";
import "mdui/components/button";

const taskQueueStore = useTaskQueueStore();

const onTop = ref(false);

onMounted(() => {
    taskQueueStore.updateQueue();
});

watch(onTop, (value) => {
    setWindowOnTop(value);
});

function setWindowOnTop(onTop: boolean) {
    invoke("set_window_on_top", {
        onTop: onTop,
    });
}

function closeWindow() {
    const mainWindow = new WebviewWindow("main");
    mainWindow.emit("mini_window_close");
    CommandInvoker.closeWindow();
}
</script>

<template>
    <div class="select-none container h-screen">
        <div
            class="relative flex justify-center items-center error-container h-10 close"
        >
            <mdui-button variant="text" class="error-text" @click="closeWindow"
                >Close</mdui-button
            >
        </div>
        <div
            class="w-full flex items-center justify-center"
            v-if="!taskQueueStore.queueRunning"
        >
            <p class="text-center text-gray-500">Queue Stopped</p>
        </div>
        <div class="flex flex-col" v-else>
            <mdui-list>
                <mdui-list-item v-for="task in taskQueueStore.taskQueue">
                    <div class="items-center align-middle justify-center">
                        <mdui-circular-progress
                            v-if="task.state === 'Running'"
                        ></mdui-circular-progress>
                        <mdui-icon
                            size="small"
                            v-else-if="task.state === 'Failed'"
                        >
                            error
                        </mdui-icon>
                        <mdui-icon
                            size="small"
                            v-else-if="task.state === 'Completed'"
                        >
                            done
                        </mdui-icon>
                        <p>{{ task.taskType }}</p>
                    </div>
                </mdui-list-item>
            </mdui-list>
        </div>

        <div
            class="flex fixed justify-center items-center gap-2 w-full bottom-4 hover:cursor-pointer"
            @click="onTop = !onTop"
        >
            <mdui-checkbox :checked="onTop" />
            Always on top
        </div>
    </div>
</template>

<style scoped>
.close {
    transform: translateY(-100%);
    display: none;
}

.container:hover > .close {
    transform: translateY(0);
    animation: close_in 0.1s;
    display: flex;
}

@keyframes close_in {
    0% {
        transform: translateY(-100%);
        height: 0;
    }

    100% {
        transform: translateY(0);
        height: 1.5rem;
    }
}
</style>
