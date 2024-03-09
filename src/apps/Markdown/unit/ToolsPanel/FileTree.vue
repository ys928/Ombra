<template>
    <div class="FileTree">
        <div class="title">资源管理器</div>

        <div class="tree" @click="fun_click_select_folder">
            <DirNode v-if="prop.root.length != 0" :is_open="true" :name="fun_name(prop.root)" :path="prop.root"
                :fun_open_dir="prop.fun_opendir" :level="0"></DirNode>
            <Blank v-else></Blank>
        </div>

        <div class="tools">

        </div>
    </div>
</template>


<script setup lang="ts">
import DirNode from './FileTree/DirNode.vue';
import Blank from './FileTree/Blank.vue';

const prop = defineProps(['fun_opendir', 'root', 'is_show', 'select_dir']);
//从路径中解析出来最后的文件名称
function fun_name(path: string) {
    let p = path.replace(/\\/g, '/'); //将所有\替换为/
    let arr = p.split('/');
    return arr.pop();
}

function fun_click_select_folder() {
    if (prop.root.length == 0) {
        prop.select_dir();
    }
}

</script>

<style scoped lang="less">
.FileTree {
    width: 100%;
    height: 100%;
    background-color: #212121;
    color: #b6b6b6;
    display: flex;
    flex-direction: column;
    user-select: none;

    .title {
        font-size: 12px;
        margin: 5px 10px;
        font-weight: bold;
    }

    .tree {
        height: 100px;
        flex-grow: 1;
        overflow: auto;
        cursor: pointer;

        &::-webkit-scrollbar {
            width: 5px;
            height: 5px;
        }

        &::-webkit-scrollbar-thumb {
            background-color: #434343;
            border-radius: 3px;
        }

        &::-webkit-scrollbar-track {
            background-color: #323232;
        }
    }

    .tools {
        height: 20px;
        background-color: #292a2d;
    }
}
</style>