<script setup lang="ts">
import TaskStatus from "@/interface/TaskStatus";
import { computed, ref } from "vue";

const props = defineProps<{
    index: number;
    task: TaskStatus;
}>();

const showRemoveButton = ref(false);

function mouseEnter() {
    if (props.task.state !== "Running") {
        showRemoveButton.value = true;
    }
}

function mouseLeave() {
    showRemoveButton.value = false;
}

const backgroundColor = computed(() => {
    if (props.task.state === "Running") {
        return "#FFD700";
    } else if (props.task.state === "Completed") {
        return "#00FF00";
    } else if (props.task.state === "Failed") {
        return "#FF0000";
    } else {
        return "#EBEBE4";
    }
});
</script>

<template>
    <div
        @mouseenter="mouseEnter"
        :style="{ backgroundColor: backgroundColor }"
        @mouseleave="mouseLeave"
        ref="outer"
        class="item mx-1 text-center items-center shadow relative"
    >
        <mdui-card clickable class="flex flex-col w-full h-full layer">
            <p class="text-center">{{ props.task.taskType }}</p>
            <mdui-linear-progress
                v-if="props.task.state === 'Running'"
                class="w-11/12"
            ></mdui-linear-progress>
        </mdui-card>
    </div>
</template>

<style scope>
.item {
    min-width: 150px;
    width: 150px;
    height: 60px;
    border-radius: 0.5rem;
    background-color: var(--md-ref-palette-neutral95);
}
</style>
