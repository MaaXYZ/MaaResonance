<script setup lang="ts">
import { TaskType, allTaskTypes, allTaskTypesContent } from "@/interface/TaskStatus";
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
            <mdui-tab v-for="(name,taskType) in allTaskTypesContent" :value="taskType">{{ $t(name) }}</mdui-tab>

            <mdui-tab-panel slot="panel" v-for="taskType in allTaskTypes" :value="taskType">
                <component :is="tabs[taskType]" />
            </mdui-tab-panel>
        </mdui-tabs>
    </mdui-card>
</template>
