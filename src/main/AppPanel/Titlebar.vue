<script setup lang="ts">
import { KIMinus, KIFullScreen, KIClose, KISeparate } from '~/kui'
import { onMounted, onUnmounted, ref, watch } from 'vue';
import Window from '~/api/window';
import GlobalShortcut from '~/api/globalShortcut';
import Ombra from '~/api/ombra';
import App from '~/api/app';
import Config from '~/api/config';
//窗口是否显示
const is_show = ref(true);
const callout_short_key = ref('');

async function winSeparate() {
    let appid = Ombra.get_appid();
    if (appid.length > 0) {
        Window.new_app(appid);
    }
}
let fun_eve_blur: Promise<Function>;
let fun_eve_focus: Promise<Function>;
if (Window.is_main()) {
    fun_eve_blur = Window.event_blur('MainWindow', () => {
        is_show.value = false;
    })
    fun_eve_focus = Window.event_focus('MainWindow', () => {
        is_show.value = true;
    });
    let timer: NodeJS.Timeout | undefined;
    watch(is_show, () => {
        if (timer) clearTimeout(timer);
        timer = setTimeout(async () => {
            let is_visible = await Window.is_visible();
            if (is_visible != is_show.value) {
                if (is_show.value) {
                    Window.show();
                    Window.focus();
                } else {
                    Window.hide();
                }
            }
        }, 100);
    });
}


onMounted(async () => {
    Window.shadow();
    if (Window.is_main()) {
        //读取唤出面板的快捷键
        callout_short_key.value = await Config.read_item('callout');
        if (callout_short_key.value == undefined) {
            callout_short_key.value = 'CommandOrControl+Shift+A';
            set_callout_shortkey('CommandOrControl+Shift+A');
        } else {
            set_callout_shortkey(callout_short_key.value);
        }
    }
});
onUnmounted(() => {
    if (Window.is_main()) {
        fun_eve_blur.then((fun) => {
            fun();
        });
        fun_eve_focus.then((fun) => {
            fun();
        });
    }
});
//处理程序退出时的情况
async function WinClose() {
    Window.close();
}
async function WinMin() {
    Window.min();
}
async function WinTogMax() {
    Window.toggle_max();
}

async function set_callout_shortkey(shortkey: string) {
    GlobalShortcut.auto_set(shortkey, async () => {
        if (is_show.value) {
            is_show.value = false;
        } else {
            is_show.value = true;
        }
    })
    Config.write_item('callout', shortkey);
    callout_short_key.value = shortkey;
}

</script>

<template>
    <div class="Titlebar" data-tauri-drag-region>
        <div class="Icon">
            <img src="/logo.png">
        </div>
        <div class="mmc">
            <KISeparate class="sep" :w="12" :h="12" @click="winSeparate" v-if="App.is_embed()" title="分离窗口">
            </KISeparate>
            <KIMinus class="min" w="12" h="12" @click="WinMin"></KIMinus>
            <KIFullScreen class="max" w="12" h="12" @click="WinTogMax"></KIFullScreen>
            <KIClose class="close" w="12" h="12" @click="WinClose"></KIClose>
        </div>
    </div>
</template>

<style scoped lang="less">
.Titlebar {
    height: 35px;
    line-height: 35px;
    display: flex;
    justify-content: space-between;

    .Icon {
        height: 35px;
        display: flex;
        align-items: center;
        margin-left: 10px;
        user-select: none;

        img {
            width: 16px;
            height: 16px;
        }
    }

    .search {
        margin: 6px 0;
        height: 23px;
        width: 250px;
        outline: none;
        color: #c2c2c2;
        border: 1px solid #454545;
        background-color: #242424;
        border-radius: 5px;
        padding: 0 10px;
        user-select: none;
    }

    .mmc {
        height: 35px;
        line-height: 35px;
        display: flex;
        align-items: center;

        .sep,
        .max,
        .min {
            width: 40px;
            text-align: center;
            color: #cccccc;
            cursor: pointer;

            &:hover {
                background-color: #373737;
            }
        }

        .close {
            width: 40px;
            text-align: center;
            color: #cccccc;
            cursor: pointer;

            &:hover {
                background-color: #e81123;
            }
        }
    }
}
</style>