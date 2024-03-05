<script setup lang="ts">
import Labels from './MainPanel/Labels.vue';
import Page from './MainPanel/Page.vue';
import { useOpenFilesStore } from '../stores/openfiles'
import { onMounted } from 'vue';

const openfilesStore = useOpenFilesStore();

onMounted(() => {
    if (openfilesStore.mdfiles.length == 0) {
        openfilesStore.open_default();
    }
});

</script>


<template>
    <div class="MainPanel">
        <Labels></Labels>
        <template v-for="(item, index) in openfilesStore.mdfiles" :key="item.path">
            <Page v-if="index == openfilesStore.show_index" :md_file="item"></Page>
        </template>
    </div>
</template>

<style scoped lang="less">
.MainPanel {
    display: flex;
    flex-direction: column;
    height: 100%;

    .Labels {
        height: 30px;
    }

    .Page {
        height: 100px;
        flex-grow: 1;
    }
}
</style>