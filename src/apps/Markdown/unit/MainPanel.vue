<script setup lang="ts">
import { Ref, inject, onUpdated } from 'vue';
import Labels from './MainPanel/Labels.vue';
import Page from './MainPanel/Page.vue';

type md_file = {
    name: string,
    path: string,
    content: string
};
const md_files = inject('md_files') as Array<md_file>;
const cur_show_file = inject('cur_show_file') as Ref<number>;
const next_show_file = inject('next_show_file') as Ref<number>;

onUpdated(() => {
    console.log(next_show_file.value, cur_show_file.value);
    if (next_show_file.value != cur_show_file.value) {
        cur_show_file.value = next_show_file.value;
    }
});

</script>


<template>
    <div class="MainPanel">
        <Labels></Labels>
        <template v-for="(item, index) in md_files" :key="item.path">
            <Page v-if="index == cur_show_file" :md_file="item"></Page>
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