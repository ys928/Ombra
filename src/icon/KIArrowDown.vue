<template>
    <i class="KIcon">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24"
            :width="prop.w" :height="prop.h">
            <path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6l-6-6l1.41-1.41z" fill="currentColor"></path>
        </svg>
    </i>
</template>

<script setup lang="ts">
const prop = defineProps(['w', 'h']);

</script>

<style scoped lang="less"></style>