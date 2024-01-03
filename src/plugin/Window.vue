<template>
    <div class="PluginWindow">
        <AppTitlebar></AppTitlebar>
        <iframe id="plugin" :src="f" frameborder="0"></iframe>
    </div>
</template>

<script setup lang="ts">
import { file_convert, file_read_text, file_write_text } from '~/ombra';
import AppTitlebar from '../components/AppTitlebar.vue'
import { onMounted } from 'vue';

let f = file_convert('D:\\Desktop\\batch-rename\\dist\\index.html');

let iframe: HTMLIFrameElement;
onMounted(() => {
    iframe = document.querySelector('#plugin') as HTMLIFrameElement;
    window.addEventListener('message', handle_plugin);
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