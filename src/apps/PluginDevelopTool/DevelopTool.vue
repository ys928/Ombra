<template>
    <div class="DevelopTool">
        <AppTitlebar></AppTitlebar>
        <iframe id="plugin" :src="plugin_index" frameborder="0"></iframe>
    </div>
</template>

<script setup lang="ts">

import { onMounted, onUnmounted, ref } from 'vue';
import AppTitlebar from '~/components/AppTitlebar.vue';
import { file_read_text, file_write_text } from '~/ombra';

let plugin_index = ref('');
let iframe: HTMLIFrameElement;

onMounted(() => {
    iframe = document.querySelector('#plugin') as HTMLIFrameElement;
    window.addEventListener('message', handle_plugin);
});

onUnmounted(() => {
    window.removeEventListener('message', handle_plugin);
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
</script>

<style scoped lang="less">
.PluginWindow {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;

    iframe {
        width: 100vw;
        height: 100px;
        flex-grow: 1;
    }
}
</style>