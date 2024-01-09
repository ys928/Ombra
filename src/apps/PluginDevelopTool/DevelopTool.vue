<template>
    <div class="DevelopTool">
        <AppTitlebar></AppTitlebar>
        <iframe v-if="show_plugin_index" id="plugin" :src="plugin_index" frameborder="0"></iframe>
        <div class="menu">
            <div class="item" @click="fun_choose">选择</div>
        </div>
    </div>
</template>

<script setup lang="ts">

import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { nextTick, onMounted, onUnmounted, ref } from 'vue';
import AppTitlebar from '~/components/AppTitlebar.vue';
import { dlg_open, file_convert, file_read_text, file_write_text } from '~/ombra';

let plugin_index = ref(''); //转换后的路径
let raw_plugin_index = '';//原始文件路径备份
const show_plugin_index = ref(true);

let iframe: HTMLIFrameElement;

onMounted(() => {
    iframe = document.querySelector('#plugin') as HTMLIFrameElement;
    window.addEventListener('message', handle_plugin);
});

onUnmounted(() => {
    window.removeEventListener('message', handle_plugin);
    if (raw_plugin_index.length > 0) {
        invoke('unwatch_dir', { path: raw_plugin_index });
    }
});

async function handle_plugin(e: MessageEvent) {
    if (e.data.type == 'func') {
        switch (e.data.name) {
            case 'file_read_text':
                {
                    let text = await file_read_text(e.data.path);
                    iframe.contentWindow?.postMessage(text, '*');
                }
                break;
            case 'file_write_text':
                {
                    await file_write_text(e.data.path, e.data.content);
                    iframe.contentWindow?.postMessage('', '*');
                }
                break;
        }
    }
}

async function fun_choose() {
    let paths = await dlg_open({
        multiple: false, directory: false, filters: [{
            name: 'html',
            extensions: ['html', 'htm']
        }]
    });
    if (paths == null) return;
    if (Array.isArray(paths)) return;
    if (raw_plugin_index.length > 0) {
        invoke('unwatch_dir', { path: raw_plugin_index });
    }
    raw_plugin_index = paths;
    plugin_index.value = file_convert(paths);
    // localStorage.setItem('raw_plugin_index', raw_plugin_index);
    invoke("watch_dir", { path: paths });
}

listen('file_watch', async (e) => {
    if (e.payload == 'modify') {
        //文件一旦变化，则自动刷新页面数据
        show_plugin_index.value = false;
        await nextTick();
        show_plugin_index.value = true;
        // plugin_index.value = file_convert(raw_plugin_index);
        // window.location.reload();
    }
})

</script>

<style scoped lang="less">
.DevelopTool {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;

    iframe {
        width: 100vw;
        height: 100px;
        flex-grow: 1;
    }

    .menu {
        position: fixed;
        bottom: 10px;
        left: 50%;
        transform: translateX(-50%);

        .item {
            background-color: #2a2a2a;
            color: #c9d1d9;
            border-radius: 5px;
            user-select: none;
            cursor: pointer;
            padding: 5px;

            &:hover {
                background-color: #30363d;
            }
        }
    }
}
</style>