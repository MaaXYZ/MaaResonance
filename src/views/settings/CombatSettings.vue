<script setup lang="ts">
import { useMaaStateStore } from "@/stores/MaaStateStore";
const maaStateStore = useMaaStateStore();

const config = maaStateStore.config?.combat;
</script>

<template>
    <div class="flex justify-center items-center">
        <mdui-list class="w-full">
            <mdui-list-item class="settings_item" nonclickable>
                <div class="flex flex-row justify-around">
                    <p class="label">{{ $t("settings.times") }}</p>
                    <mdui-text-field
                        :value="config?.times"
                        class="option"
                        type="number"
                        @change="
                            maaStateStore.setCombatTimes(+$event.target.value)
                        "
                    >
                    </mdui-text-field>
                </div>
            </mdui-list-item>
        </mdui-list>
    </div>
</template>

<style>
mdui-select,
mdui-text-field {
    width: 300px;
}
</style>
