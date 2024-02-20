<template>
    <i class="KIcon">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"
            :width="prop.w" :height="prop.h">
            <path d="M15.41 16.59L10.83 12l4.58-4.59L14 6l-6 6l6 6l1.41-1.41z" fill="currentColor"></path>
        </svg>
    </i>
</template>

<script setup lang="ts">
const prop = defineProps(['w', 'h']);

</script>

<style scoped lang="less"></style>