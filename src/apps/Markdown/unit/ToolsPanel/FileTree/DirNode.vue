<template>
    <div class="DirNode">
        <div class="filename" @click="fun_dir">
            <div class="show" :style="{ left: prop.level * 10 + 'px' }">
                <KIArrowDown w='18' h="18" v-if="is_open"></KIArrowDown>
                <KIArrowRight w='18' h="18" v-else></KIArrowRight>
                <span>{{ prop.name }}</span>
            </div>
        </div>
        <div class="children" v-if="is_open">
            <template v-for="item in children.dir">
                <DirNode :is_open="false" :name="item.name" :path="item.path" :fun_open_dir="prop.fun_open_dir"
                    :level="prop.level + 1">
                </DirNode>
            </template>
            <div v-for="item in children.file" class="filename" @dblclick="fun_openfile(item.path)">
                <div class="show" :style="{ left: prop.level * 10 + 15 + 'px' }">
                    <KIMarkdown w="16" h="16" v-if="item.name.endsWith('.md')"></KIMarkdown>
                    <KIText w="16" h="16" v-else></KIText>
                    <span class="name">{{ item.name }}</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { KIArrowRight, KIArrowDown, KIText, KIMarkdown } from '~/icon'
import { onMounted, reactive, ref } from 'vue';
import { useOpenFilesStore } from '../../../stores/openfiles';

const prop = defineProps(['name', 'path', 'is_open', 'fun_open_dir', 'level']);

type FileInfo = {
    path: string,
    name: string,
};
type FileChildren = {
    dir: Array<FileInfo>,
    file: Array<FileInfo>
};
const is_open = ref(false);

const openfilesStore = useOpenFilesStore();

const children = reactive({
    dir: [],
    file: []
}) as FileChildren;

onMounted(() => {
    //如果默认打开
    if (prop.is_open) {
        fun_dir();
    }
});

async function fun_openfile(p: string) {
    openfilesStore.open(p);
}

async function fun_dir() {
    if (is_open.value) {
        is_open.value = false;
    } else {
        const ret = await prop.fun_open_dir(prop.path);
        children.dir = ret.dir;
        children.file = ret.file;
        is_open.value = true;
    }
}
</script>

<style scoped lang="less">
.DirNode {
    width: 100%;
    background-color: #212121;
    color: #b6b6b6;
    overflow: hidden;

    .filename {
        position: relative;
        width: 100%;
        height: 25px;

        &:hover {
            background-color: #2f2f2f;
        }

        .show {
            font-size: 13px;
            height: 25px;
            cursor: pointer;
            user-select: none;
            display: flex;
            align-items: center;
            position: absolute;
            white-space: nowrap;
        }
    }

}
</style>../../../stores/openfiles