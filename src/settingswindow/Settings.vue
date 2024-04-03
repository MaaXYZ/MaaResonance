<script setup lang="ts">
import { useMaaStateStore } from "@/stores/MaaStateStore";
import { defineAsyncComponent, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";

const maaStateStore = useMaaStateStore();

const { t } = useI18n();

const settingSections = {
    App: {
        component: defineAsyncComponent(() => import("./AppSettings.vue")),
        name: t("settings.appSettings"),
    },
};

type SettingSections = keyof typeof settingSections;

const activeSetting = ref<SettingSections>("App");

onMounted(() => {
    maaStateStore.getConfig();
});
</script>

<template>
    <div class="container flex flex-row h-screen w-screen select-none">
        <mdui-list class="sidebar">
            <mdui-list-item
                v-for="(content, name) in settingSections"
                :key="name"
                rounded
                :active="activeSetting == name"
                @click="activeSetting = name"
                :headline="content.name"
            >
            </mdui-list-item>
        </mdui-list>
        <div class="divider"></div>
        <component
            :is="settingSections[activeSetting].component"
            class="flex-grow"
        />
    </div>
</template>

<style scoped>
.sidebar {
    width: 200px;
}

.divider {
    width: 1px;
    background-color: #e0e0e0;
}
</style>
