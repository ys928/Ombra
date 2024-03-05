<script setup lang="ts">
import { Ref, ref } from 'vue';
import { Path, Explorer, Clipboard } from '~/api'
import { usePopMenuStore } from '../stores/popMenu';

const popMenuStore = usePopMenuStore();

const div_pop_menu = ref() as Ref<HTMLElement>

async function fun_open_path() {
    popMenuStore.set_show(false);

    let p = popMenuStore.click_item.name;
    if (popMenuStore.click_item.ext.length > 0) {
        p += '.' + popMenuStore.click_item.ext;
    }
    if (popMenuStore.click_item.path.length != 0) {
        p = await Path.join(popMenuStore.click_item.path, p);
    }
    console.log(p);
    Explorer.select_file(p);
}

async function fun_open_file() {
    popMenuStore.set_show(false);
    let p = popMenuStore.click_item.name;
    if (popMenuStore.click_item.ext.length > 0) {
        p += '.' + popMenuStore.click_item.ext;
    }
    if (popMenuStore.click_item.path.length != 0) {
        p = await Path.join(popMenuStore.click_item.path, p);
    }
    Explorer.open_file(p);
}

async function fun_copy_path() {
    popMenuStore.set_show(false);
    let p = popMenuStore.click_item.name;
    if (popMenuStore.click_item.ext.length > 0) {
        p += '.' + popMenuStore.click_item.ext;
    }
    if (popMenuStore.click_item.path.length != 0) {
        p = await Path.join(popMenuStore.click_item.path, p);
    }
    Clipboard.set_text(p);
}
</script>

<template>
    <div class="PopMenu" ref="div_pop_menu" v-if="popMenuStore.show"
        :style="{ 'left': popMenuStore.x + 'px', 'top': popMenuStore.y + 'px' }">
        <div class="item" @click="fun_open_file">打开</div>
        <div class="item" @click="fun_open_path">打开路径</div>
        <div class="item" @click="fun_copy_path">复制路径</div>
    </div>
</template>

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