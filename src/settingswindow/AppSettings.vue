<script setup lang="ts">
import { useMaaStateStore } from "@/stores/MaaStateStore";
import {
    maaAdbControllerTouchTypes,
    maaAdbControllerScreencapTypes,
} from "@/interface/AppConfig";

const maaStateStore = useMaaStateStore();
const config = maaStateStore.config?.appConfig;
</script>

<template>
    <div>
        <mdui-list>
            <mdui-list-item nonclickable class="settings_item">
                <div>
                    <p class="label">Adb Touch Type</p>
                    <mdui-select
                    class="option"
                        :value="config?.adb_controller_type.touch_type"
                    >
                        <mdui-menu-item
                            v-for="ty in maaAdbControllerTouchTypes"
                            @change="maaStateStore.setControllerTouchType(ty)"
                            :key="ty"
                            :value="ty"
                        >
                            {{ ty }}
                        </mdui-menu-item>
                    </mdui-select>
                </div>
            </mdui-list-item>
            <mdui-list-item nonclickable class="settings_item">
                <div>
                    <p class="label">Adb Screencap Type</p>
                    <mdui-select
                        class="option"
                        :value="config?.adb_controller_type.screencap_type"
                    >
                        <mdui-menu-item
                            v-for="ty in maaAdbControllerScreencapTypes"
                            @change="
                                maaStateStore.setControllerScreencapType(ty)
                            "
                            :key="ty"
                            :value="ty"
                        >
                            {{ ty }}
                        </mdui-menu-item>
                    </mdui-select>
                </div>
            </mdui-list-item>
        </mdui-list>
    </div>
</template>

<style>
mdui-select {
    width: 300px;
}

.settings_item {
    margin-bottom: 10px;
}

.label {
    position: absolute;
    margin-left: 10px;
}

.option {
    position: absolute;
    right: 0;
}

</style>