<script setup lang="ts">
import { useRoute } from 'vue-router';
import Titlebar from './AppPanel/Titlebar.vue'
import { Ref, onMounted, onUnmounted, ref, h, render, nextTick } from 'vue';
import { FS, Window } from '~/api';
import { useAppListStore } from '~/stores/appList';
import { load_apps } from '~/apps';

const route = useRoute();


const app_content = ref() as Ref<HTMLElement>;

const applistStore = useAppListStore();

let iframe: HTMLIFrameElement;

onMounted(async () => {
    let id = route.query.id;
    if (!Window.is_main()) {
        await load_apps();
    }
    let app_list = applistStore.applist;
    // console.log(app_list);
    for (let a of app_list) {
        if (a.id != id) continue;
        // console.log(a);
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
            render(h(a.component), app_content.value);
        }
        break;
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
                    let text = await FS.read_text(e.data.path);
                    iframe.contentWindow?.postMessage(text, '*');
                }
                break;
            case 'file_write_text':
                {
                    await FS.write_text(e.data.path, e.data.content);
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