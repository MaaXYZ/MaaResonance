<script setup lang="ts">
import CloseIcon from "@/assets/icons/CloseIcon.vue";
import DoneIcon from "@/assets/icons/DoneIcon.vue";
import TaskStatus, { allTaskTypesContent } from "@/interface/TaskStatus";
import { useTaskQueueStore } from "@/stores/TaskQueueStore";

const taskQueueStore = useTaskQueueStore();

const props = defineProps<{
    index: number;
    task: TaskStatus;
}>();

function removeCurrent() {
    taskQueueStore.removeFromQueue(props.index);
}
</script>

<template>
    <div ref="outer" class="item mx-1 text-center items-center">
        <mdui-dropdown
            :trigger="task.state === 'Running' ? 'manual' : 'contextmenu'"
            placement="bottom-end"
        >
            <mdui-card
                :clickable="task.state === 'Pending'"
                slot="trigger"
                class="flex flex-col w-full h-full layer"
            >
                <div class="flex flex-row items-center justify-center">
                    <mdui-icon
                        v-if="task.state === 'Completed'"
                        style="color: green"
                    >
                        <DoneIcon />
                    </mdui-icon>
                    <mdui-icon
                        v-if="task.state === 'Failed'"
                        style="color: red"
                    >
                        <CloseIcon />
                    </mdui-icon>
                    <p>
                        {{ $t(allTaskTypesContent[props.task.taskType]) }}
                    </p>
                </div>
                <mdui-linear-progress
                    v-if="props.task.state === 'Running'"
                ></mdui-linear-progress>
            </mdui-card>
            <mdui-menu>
                <mdui-menu-item @click="removeCurrent">
                    {{ $t("remove") }}
                </mdui-menu-item>
            </mdui-menu>
        </mdui-dropdown>
    </div>
</template>

<style scope>
.item {
    min-width: 150px;
    width: 150px;
    height: 60px;
}
</style>
