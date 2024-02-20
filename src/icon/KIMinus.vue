<template>
    <i class="KIcon">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" :width="prop.w" :height="prop.h">
            <path fill="currentColor" d="M128 544h768a32 32 0 1 0 0-64H128a32 32 0 0 0 0 64"></path>
        </svg>
    </i>
</template>

<script setup lang="ts">
const prop = defineProps(['w', 'h']);

</script>

<style scoped lang="less"></style>