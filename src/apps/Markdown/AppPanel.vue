<script setup lang="ts">
import MainPanel from './unit/MainPanel.vue'
import ToolsPanel from './unit/ToolsPanel.vue';
</script>


<template>
    <div class="AppPanel">
        <ToolsPanel></ToolsPanel>
        <MainPanel></MainPanel>
    </div>
</template>

<style scoped lang="less">
.AppPanel {
    height: 100%;
    width: 100%;
    display: flex;

    .ToolsPanel {
        width: 250px;
        height: 100%;
    }

    .MainPanel {
        width: 100px;
        flex-grow: 1;
    }

}
</style>