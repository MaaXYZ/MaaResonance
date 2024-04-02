<script setup lang="ts">
import TaskQueueItem from '@/components/TaskQueueItem.vue';
import { useTaskQueueStore } from '@/stores/TaskQueueStore';

const taskQueueStore = useTaskQueueStore();

</script>

<template>
        <div  v-if="taskQueueStore.taskQueue.length>0" class="flex overflow-x-auto overflow-y-hidden align-middle items-center">
            <TaskQueueItem v-for="(task, index) in taskQueueStore.taskQueue" :key="index" :index="index" :task="task" />
        </div>
        <div v-else class="flex justify-center items-center">
            <p class="text-center text-gray-400 text-4xl">
                No tasks in queue
            </p>
        </div>
</template>