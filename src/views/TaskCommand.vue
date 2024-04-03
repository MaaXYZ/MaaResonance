<script setup lang="ts">
import { useTaskQueueStore } from "@/stores/TaskQueueStore";
import { computed } from "vue";
import { TaskType, allTaskTypes } from "@/interface/TaskStatus";
import CommandInvoker from "@/CommandInvoker";
import { useMaaStateStore } from "@/stores/MaaStateStore";
import PlayArrowIcon from "@/assets/icons/PlayArrowIcon.vue";
import StopIcon from "@/assets/icons/StopIcon.vue";
import { snackbar } from "mdui/functions/snackbar";

const taskQueueStore = useTaskQueueStore();
const maaStateStore = useMaaStateStore();

const taskQueueActionText = computed(() => {
    if (taskQueueStore.queueRunning) {
        return "Stop";
    } else {
        return "Start";
    }
});

function queueAction() {
    console.log("queueAction");
    if (taskQueueStore.queueRunning) {
        taskQueueStore.stopQueue();
    } else if (taskQueueStore.hasPendingTasks) {
        taskQueueStore.startQueue();
    } else {
        snackbar({
            message: "No tasks to run",
        });
    }
}

function addTask(task: TaskType) {
    taskQueueStore.addToQueue(task).catch((err) => {
        snackbar({
            message: err.message,
        });
    });
}

function startMiniWindow() {
    CommandInvoker.startMiniWindow()
        .then(() => {
            maaStateStore.miniWindowOpened = true;
        })
        .catch((e) => {
            console.error(e);
            snackbar({
                message: "Failed to start mini window",
            });
        });
}
</script>

<template>
    <div class="-mr-3 rounded-lg bg-white p-2 pt-4">
        <mdui-button class="w-full" @click="queueAction"
            >{{ taskQueueActionText }}
            <mdui-icon slot="icon">
                <StopIcon v-if="taskQueueStore.queueRunning"/>
                <PlayArrowIcon v-else/>
            </mdui-icon>
        </mdui-button>

        <!-- TODO: figure out how to change state with mini window open state -->
        <mdui-button variant="outlined" class="w-full mt-2" @click="startMiniWindow"
            >Mini Window</mdui-button
        >
        <div class="h-2" />
        <mdui-button variant="tonal"
            secondary
            type="primary"
            class="w-full mb-2"
            v-for="task in allTaskTypes"
            :key="task"
            @click="addTask(task)"
        >
            {{ task }}
        </mdui-button>
    </div>
</template>

<style scoped></style>
