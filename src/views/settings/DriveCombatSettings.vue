<script setup lang="ts">
import { useMaaStateStore } from "@/stores/MaaStateStore";
const maaStateStore = useMaaStateStore();

const config = maaStateStore.config?.driveCombat;
</script>

<template>
    <div class="flex justify-center items-center">
        <mdui-list class="w-full">
            <mdui-list-item class="settings_item" nonclickable>
                <div class="flex flex-row justify-around items-center">
                    <p class="label">{{ $t("settings.useFuel") }}</p>
                    <mdui-switch
                        :checked="config?.use_fuel"
                        @change="
                            maaStateStore.setConfig(
                                'use_fuel',
                                $event.target.checked
                            )
                        "
                    ></mdui-switch>
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
