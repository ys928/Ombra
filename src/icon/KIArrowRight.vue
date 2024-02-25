<template>
    <i class="KIcon">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"
            :width="prop.w" :height="prop.h">
            <path d="M8.59 16.59L13.17 12L8.59 7.41L10 6l6 6l-6 6l-1.41-1.41z" fill="currentColor"></path>
        </svg>
    </i>
</template>

<script setup lang="ts">
const prop = defineProps(['w', 'h']);

</script>

<style scoped lang="less">
.KIcon {
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>