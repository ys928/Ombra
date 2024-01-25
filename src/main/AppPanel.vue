<script setup lang="ts">
import { useRoute } from 'vue-router';
import Titlebar from './AppPanel/Titlebar.vue'
import { Ref, onMounted, onUnmounted, ref, h, render, nextTick } from 'vue';
import App from '~/api/app';
import File from '~/api/file';
const route = useRoute();

const app_content = ref() as Ref<HTMLElement>;

let iframe: HTMLIFrameElement;

onMounted(async () => {
    let id = route.query.id;
    let app_list = await App.get_applist();
    for (let a of app_list) {
        if (a.id != id) continue;
        console.log(a);
        if (a.component == null) {
            break;
        }
        if (typeof (a.component) == 'string') {
            const el = h('iframe', {
                src: a.component,
                frameborder: '0'
            });
            render(el, app_content.value);
            await nextTick();
            window.addEventListener('message', handle_plugin);

        } else {
            const el = h(a.component);
            render(el, app_content.value);
        }
    }
});

onUnmounted(() => {
    window.removeEventListener('message', handle_plugin);
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

</script>

<template>
    <div class="AppPanel">
        <Titlebar></Titlebar>
        <div class="content" ref="app_content">
        </div>
    </div>
</template>

<style scoped lang="less">
.AppPanel {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;

    .content {
        width: 100%;
        height: 100px;
        flex-grow: 1;

        .iframe {
            width: 100%;
            height: 100%;
        }
    }
}
</style>