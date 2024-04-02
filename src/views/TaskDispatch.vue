<script setup lang="ts">
import { onMounted } from "vue";
import { useMaaStateStore } from "@/stores/MaaStateStore";
import TaskQueue from "./TaskQueue.vue";
import TaskSettings from "./TaskSettings.vue";
import TaskCommand from "./TaskCommand.vue";
import { useToast } from "vue-toast-notification";

const maaStateStore = useMaaStateStore();

const toast = useToast();

onMounted(() => {
    maaStateStore.getConfig().catch((err) => {
        toast.error("Failed to get MAA config: " + err);
    });
});
</script>

<template>
    <div v-if="maaStateStore.isMaaReady" class="flex flex-col">
        <task-queue class="w-full h-2/5" />
        <div class="flex w-full gap-2 h-full">
            <task-command class="w-1/5 h-fit"/>
            <task-settings class="flex-grow h-fit" />
        </div>
    </div>
    <div v-else class="h-1/3" justify="center">
        <p class="text-center text-gray-400 text-4xl">
            MAA is not ready for tasks
        </p>
    </div>
</template>
