<template>
    <div class="AppTitlebar" data-tauri-drag-region>
        <div class="Icon">
            <img src="/logo.png">
        </div>
        <div class="mmc">
            <KISeparate class="sep" :w="12" :h="12" @click="winSeparate" v-if="app_is_embed()" title="分离窗口">
            </KISeparate>
            <KIMinus class="min" w="12" h="12" @click="WinMin"></KIMinus>
            <KIFullScreen class="max" w="12" h="12" @click="WinTogMax"></KIFullScreen>
            <KIClose class="close" w="12" h="12" @click="WinClose"></KIClose>
        </div>
    </div>
</template>

<script setup lang="ts">
import { KIMinus, KIFullScreen, KIClose, KISeparate } from '~/kui'
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { gs_is_registered, gs_unregister, win_close, win_min, win_new_app, win_shadow, win_toggle_max, app_is_embed, om_get_appid, win_is_main, win_event_blur, win_event_focus, win_is_visible, win_focus, win_hide, win_show } from '~/ombra';
import { read_config_item, set_shortcut, write_config_item } from '~/global';
//窗口是否显示
const is_show = ref(true);
const callout_short_key = ref('');

async function winSeparate() {
    let appid = om_get_appid();
    if (appid.length > 0) {
        win_new_app(appid);
    }
}
let fun_eve_blur: Promise<Function>;
let fun_eve_focus: Promise<Function>;
if (win_is_main()) {
    fun_eve_blur = win_event_blur('MainWindow', () => {
        is_show.value = false;
    })
    fun_eve_focus = win_event_focus('MainWindow', () => {
        is_show.value = true;
    });
    let timer: NodeJS.Timeout | undefined;
    watch(is_show, () => {
        if (timer) clearTimeout(timer);
        timer = setTimeout(async () => {
            let is_visible = await win_is_visible();
            if (is_visible != is_show.value) {
                if (is_show.value) {
                    win_show();
                    win_focus();
                } else {
                    win_hide();
                }
            }
        }, 100);
    });
}


onMounted(async () => {
    win_shadow();
    if (win_is_main()) {
        //读取唤出面板的快捷键
        callout_short_key.value = await read_config_item('callout');
        if (callout_short_key.value == undefined) {
            callout_short_key.value = 'CommandOrControl+Shift+A';
            set_callout_shortkey('CommandOrControl+Shift+A');
        } else {
            set_callout_shortkey(callout_short_key.value);
        }
    }
});
onUnmounted(() => {
    if (win_is_main()) {
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
    win_close();
}
async function WinMin() {
    win_min();
}
async function WinTogMax() {
    win_toggle_max();
}

async function set_callout_shortkey(shortkey: string) {
    await unset_callout_shortkey();
    set_shortcut(shortkey, async () => {
        if (is_show.value) {
            is_show.value = false;
        } else {
            is_show.value = true;
        }
    })
    write_config_item('callout', shortkey);
    callout_short_key.value = shortkey;
}

async function unset_callout_shortkey() {
    const isreg = await gs_is_registered(callout_short_key.value);
    if (isreg) {
        await gs_unregister(callout_short_key.value);
    }
}
</script>

<style scoped lang="less">
.AppTitlebar {
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