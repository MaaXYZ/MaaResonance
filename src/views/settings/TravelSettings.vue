<script setup lang="ts">
import { useMaaStateStore } from "@/stores/MaaStateStore";
import { allDestinations } from "@/interface/MaaConfig";
const maaStateStore = useMaaStateStore();

const config = maaStateStore.config?.travel;
</script>

<template>
    <div class="flex justify-center items-center">
        <mdui-list class="w-full">
            <mdui-list-item class="settings_item" nonclickable>
                <div class="flex flex-row justify-around items-center">
                    <p class="label">{{ $t("settings.destination") }}</p>
                    <mdui-select
                        :value="config?.destination"
                        @change="
                            maaStateStore.setConfig(
                                'destination',
                                $event.target.value
                            )
                        "
                    >
                        <mdui-menu-item
                            v-for="destination in allDestinations"
                            :value="destination"
                            :key="destination"
                            >{{ destination }}</mdui-menu-item
                        >
                    </mdui-select>
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
