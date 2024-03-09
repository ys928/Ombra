<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import MainPanel from './unit/MainPanel.vue'
import ToolsPanel from './unit/ToolsPanel.vue';
import { useOpenFilesStore } from './stores/openfiles';
import { Window } from '~/api'
const openFilesStore = useOpenFilesStore();

onMounted(() => {
    document.addEventListener("keydown", fun_keydown)
});

onUnmounted(() => {
    document.removeEventListener("keydown", fun_keydown)
})


function fun_keydown(e: KeyboardEvent) {
    if (e.ctrlKey && e.key == "n") {
        openFilesStore.open_default();
        e.preventDefault();
        e.stopPropagation();
    }

    if (e.key == "F11") {
        Window.toggle_max()
        e.preventDefault();
        e.stopPropagation();
    }

}



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