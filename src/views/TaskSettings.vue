<script setup lang="ts">
import { TaskType, allTaskTypes } from "@/interface/TaskStatus";
import EmptySettings from "./settings/EmptySettings.vue";
import CombatSettings from "./settings/CombatSettings.vue";

const tabs:Record<TaskType,any> = {
    StartUp: EmptySettings,
    Combat: CombatSettings,
};

</script>

<template>
    <mdui-card variant="elevated" class="mx-2 rounded-lg">
        <mdui-tabs value="StartUp" class="h-full">
            <mdui-tab v-for="taskType in allTaskTypes" :value="taskType">{{ taskType }}</mdui-tab>

            <mdui-tab-panel slot="panel" v-for="taskType in allTaskTypes" :value="taskType">
                <component :is="tabs[taskType]" />
            </mdui-tab-panel>
        </mdui-tabs>
    </mdui-card>
</template>
