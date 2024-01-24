<template>
    <div class="PopMenu" ref="div_pop_menu" :style="{ 'left': props.x + 'px', 'top': props.y + 'px' }">
        <div class="item" @click="fun_open_file">打开</div>
        <div class="item" @click="fun_open_path">打开路径</div>
        <div class="item" @click="fun_copy_path">复制路径</div>
    </div>
</template>

<script setup lang="ts">
import { Ref, ref, onMounted } from 'vue';
import { exp_select_file, exp_open_file } from '~/ombra';
import Path from '~/api/path'
import Clipboard from '~/api/clipboard';

const props = defineProps(['isdir', 'path', 'name', 'ext', 'x', 'y']);
const emits = defineEmits(['hidden']);

const div_pop_menu = ref() as Ref<HTMLElement>
onMounted(() => {
    document.addEventListener('click', (e: MouseEvent) => {
        if (div_pop_menu.value) {
            if (!div_pop_menu.value.contains(e.target as HTMLElement)) {
                emits('hidden');
            }
        }
    })
});

async function fun_open_path() {
    emits('hidden');
    let p = props.name;
    if (props.ext.length > 0) {
        p += '.' + props.ext;
    }
    if (props.path.length != 0) {
        p = await Path.join(props.path, p);
    }
    console.log(p);
    exp_select_file(p);
}

async function fun_open_file() {
    emits('hidden');
    let p = props.name;
    if (props.ext.length > 0) {
        p += '.' + props.ext;
    }
    if (props.path.length != 0) {
        p = await Path.join(props.path, p);
    }
    exp_open_file(p);
}

async function fun_copy_path() {
    emits('hidden');
    let p = props.name;
    if (props.ext.length > 0) {
        p += '.' + props.ext;
    }
    if (props.path.length != 0) {
        p = await Path.join(props.path, p);
    }
    Clipboard.set_text(p);
}
</script>

<style scoped lang="less">
.PopMenu {
    position: fixed;
    border: solid 1px #454545;
    border-radius: 5px;
    width: 200px;
    padding: 10px;
    background-color: #1f1f1f;
    font-size: 12px;
    color: #cccccc;

    .item {
        cursor: pointer;
        height: 25px;
        line-height: 25px;
        border-radius: 5px;
        padding: 0 10px;

        &:hover {
            background-color: #04395e;
        }
    }
}
</style>