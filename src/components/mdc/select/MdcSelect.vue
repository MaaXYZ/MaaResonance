<script setup lang="ts" generic="T">
import { onMounted, ref } from 'vue';
import { SelectOption, SelectVariant } from './SelectProps';
import 'mdui/components/select'
import 'mdui/components/menu-item'
import type { Select } from 'mdui/components/select'

defineProps<{
    options?: SelectOption[];
    variant?: SelectVariant;
}>();

const value = defineModel<string>()

const selectElement = ref<Select | null>(null);

onMounted(() => {
    if (selectElement.value) {
        selectElement.value.value = value.value ?? '';
        selectElement.value.addEventListener('change', ()=>{
            value.value = selectElement.value?.value as string | undefined
        })
    }
});

</script>

<template>
    <div class="container w-fit">
        <mdui-select :variant="variant" ref="selectElement" :value="value">
            <mdui-menu-item v-for="option in options" :value="option.value">
                {{ option.label }}
            </mdui-menu-item>
        </mdui-select>
    </div>
</template>