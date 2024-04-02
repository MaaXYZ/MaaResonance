<script setup lang="ts">
import { allTaskTypes } from "@/interface/TaskStatus";
import StartUpSettings from "./settings/StartUpSettings.vue";
import EmptySettings from "./settings/EmptySettings.vue";

const tabs = {
    StartUp: StartUpSettings,
    Award: EmptySettings,
};

</script>

<template>
    <div class="mx-2 rounded-lg bg-white">
        <mdui-tabs value="StartUp">
            <mdui-tab v-for="taskType in allTaskTypes" :value="taskType">{{ taskType }}</mdui-tab>

            <mdui-tab-panel slot="panel" v-for="taskType in allTaskTypes" :value="taskType">
                <component :is="tabs[taskType]" />
            </mdui-tab-panel>
        </mdui-tabs>
    </div>
</template>
