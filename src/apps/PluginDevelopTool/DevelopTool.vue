<template>
    <div class="DevelopTool">
        <iframe v-if="show_plugin_index" id="plugin" :src="plugin_index" frameborder="0"></iframe>
        <div class="menu">
            <div class="item" @click="fun_choose">选择</div>
            <div class="item" @click="fun_open_dev">调试</div>
        </div>
    </div>
</template>

<script setup lang="ts">

import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { nextTick, onMounted, onUnmounted, ref } from 'vue';
import File from '~/api/file'
import Window from '~/api/window';
import Notification from '~/api/notification';
import Dialog from '~/api/dialog';
import Url from '~/api/url';
interface FileChange {
    kind: string,
    files: Array<string>,
}
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
                    let text = await File.read_text(e.data.path);
                    iframe.contentWindow?.postMessage(text, '*');
                }
                break;
            case 'file_write_text':
                {
                    await File.write_text(e.data.path, e.data.content);
                    iframe.contentWindow?.postMessage('', '*');
                }
                break;
        }
    }
}

async function fun_choose() {
    let paths = await Dialog.open({
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
    plugin_index.value = Url.convert(paths);
    invoke("watch_dir", { path: paths });
}

listen<FileChange>('file_watch', async (e) => {
    if (e.payload.kind == 'modify' && e.payload.files.includes(raw_plugin_index)) {
        //文件一旦变化，则自动刷新页面数据
        show_plugin_index.value = false;
        await nextTick();
        show_plugin_index.value = true;
    }
})
async function fun_open_dev() {
    if (Window.is_main()) {
        let permissionGranted = await Notification.is_grant();
        if (!permissionGranted) {
            permissionGranted = await Notification.request();
        }
        if (permissionGranted) {
            Notification.send("提示", '请先点击右上角分离窗口后再使用调试功能');
        }
    } else {
        invoke('open_devtools');
    }
}

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
        display: flex;

        .item {
            background-color: #2a2a2a;
            color: #c9d1d9;
            border-radius: 5px;
            user-select: none;
            cursor: pointer;
            padding: 5px;
            margin: 5px;

            &:hover {
                background-color: #30363d;
            }
        }
    }
}
</style>