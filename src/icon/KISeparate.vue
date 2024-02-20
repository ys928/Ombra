<template>
    <i class="KIcon">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 20 20"
            :width="prop.w" :height="prop.h">
            <g fill="none">
                <path
                    d="M3 2a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H3zM2 4a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1v4a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4zm1 10v-3h1v3a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-3V4h3a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"
                    fill="currentColor"></path>
            </g>
        </svg>
    </i>
</template>

<script setup lang="ts">
const prop = defineProps({
    w: {
        type: Number,
        default: 16
    },
    h: {
        type: Number,
        default: 16
    }
});

</script>

<style scoped lang="less"></style>