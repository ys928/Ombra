<template>
    <div class="MainPanel">
        <div class="InputArea">
            <input class="search" v-model="search_content" ref="main_input" @input="fun_input"
                @keydown="fun_keydown($event)" @compositionstart="fun_ompositionstart" @compositionend="fun_ompositionend"
                :placeholder="search_input_placeholder">
            <div class="space" @mousedown="main_input.focus()" @dblclick="fun_click_space" data-tauri-drag-region></div>
            <div class="Icon" @click="fun_open_setting">
                <img src="/logo.png" draggable="false">
            </div>
        </div>
        <Setting v-if="is_show_setting" :set_callout_shortkey="set_callout_shortkey"
            :set_search_input_placeholder="set_search_input_placeholder" :callout_short_key="callout_short_key"
            :search_input_placeholder="search_input_placeholder"></Setting>
        <AppMenu v-else ref="apps_menu" :main_input="main_input" :search_content="search_content"
            v-model:cur_focus_app="cur_focus_app">
        </AppMenu>
        <div class="measure" ref="measure" style="visibility: hidden;"></div>
    </div>
</template>

<script setup lang="ts">
import { Ref, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import AppMenu from "./MainPanel/AppMenu.vue";
import { unregister, isRegistered } from '@tauri-apps/api/globalShortcut';
import { read_config_item, set_shortcut, write_config_item } from '~/global'
import Setting from './MainPanel/Setting.vue';
import { exp_get_path, win_event_blur, win_event_focus, win_is_visible, win_show, win_focus, win_hide, win_set_size } from '~/ombra'
import { onTextUpdate, readText, startListening } from "tauri-plugin-clipboard-api";
import { UnlistenFn } from "@tauri-apps/api/event";

const main_input = ref() as Ref<HTMLInputElement>;
const measure = ref() as Ref<HTMLElement>;
const search_content = ref("");

const apps_menu = ref();
const cur_focus_app = ref(0);
const is_show_setting = ref(false);
const search_input_placeholder = ref('');
const is_show = ref(true);
const callout_short_key = ref('');

win_event_blur('MainWindow', () => {
    is_show.value = false;
})
win_event_focus('MainWindow', () => {
    is_show.value = true;
});


let timer: NodeJS.Timeout | undefined;
watch(is_show, () => {
    if (timer) clearTimeout(timer);
    timer = setTimeout(async () => {
        let is_visible = await win_is_visible();
        if (is_visible != is_show.value) {
            if (is_show.value) {
                let p = await exp_get_path();
                if (p == 'none') {
                    apps_menu.value.init_feature([], '');
                } else {
                    apps_menu.value.init_feature(['explorer'], p);
                }
                win_show();
                win_focus();
            } else {
                win_hide();
            }
        }
    }, 100);

});

let unlistenTextUpdate: UnlistenFn;
let unlistenClipboard: () => Promise<void>;
let clip_board_time = 999;
let timing_fun: string | number | NodeJS.Timeout | undefined;
onMounted(async () => {
    win_set_size(170);
    main_input.value.focus();
    //读取唤出面板的快捷键
    callout_short_key.value = await read_config_item('callout');
    if (callout_short_key.value == undefined) {
        callout_short_key.value = 'CommandOrControl+Shift+A';
        set_callout_shortkey('CommandOrControl+Shift+A');
    } else {
        set_callout_shortkey(callout_short_key.value);
    }
    //读取搜索栏占位符
    let placeholder = await read_config_item('placeholder');
    if (placeholder == undefined || placeholder.length == 0) {
        placeholder = 'Hi，Ombra！';
        search_input_placeholder.value = 'Hi，Ombra！';
    } else {
        search_input_placeholder.value = placeholder;
    }

    unlistenTextUpdate = await onTextUpdate(() => {
        clip_board_time = 0;
        if (timing_fun) clearInterval(timing_fun);
        timing_fun = setInterval(() => {
            clip_board_time += 1;
            if (clip_board_time > 3 && timing_fun) {
                clearInterval(timing_fun);
            }
        }, 1000);
    });
    unlistenClipboard = await startListening();
});

onUnmounted(() => {
    unlistenTextUpdate();
    unlistenClipboard();
})

async function set_callout_shortkey(shortkey: string) {
    await unset_callout_shortkey();
    set_shortcut(shortkey, async () => {
        if (is_show.value) {
            is_show.value = false;
        } else {
            is_show.value = true;
            if (clip_board_time >= 0 && clip_board_time <= 3) {
                search_content.value = await readText();
                fun_input();
            }
        }
    })
    write_config_item('callout', shortkey);
    callout_short_key.value = shortkey;
}

async function unset_callout_shortkey() {
    const isreg = await isRegistered(callout_short_key.value);
    if (isreg) {
        await unregister(callout_short_key.value);
    }
}
async function set_search_input_placeholder(placeholder: string) {
    search_input_placeholder.value = placeholder;
    write_config_item('placeholder', search_input_placeholder.value);
}

async function fun_keydown(e: KeyboardEvent) {
    if (e.key == 'ArrowUp') {
        apps_menu.value.move('up');
        return;
    }
    if (e.key == 'ArrowDown') {
        apps_menu.value.move('down');
        return;
    }
    if (e.key == 'ArrowLeft') {
        apps_menu.value.move('left');
        return;
    }

    if (e.key == 'ArrowRight') {
        apps_menu.value.move('right');
        return;
    }
    if (e.key == 'Enter') {
        e.preventDefault();
        e.stopPropagation();
        apps_menu.value.click_app();
        return;
    }
    if (e.key == "Escape") {
        if (is_show_setting.value) {
            is_show_setting.value = false;
            return;
        }
        if (search_content.value.length != 0) {
            search_content.value = "";
            await nextTick();
            fun_input();
        } else {
            win_hide();
        }
    }
}
let is_ompositioning = false;
async function fun_input() {
    is_show_setting.value = false;
    if (is_ompositioning) return;
    adjust_input_size();
    cur_focus_app.value = 0;
    apps_menu.value.search();
}

function adjust_input_size() {
    measure.value.innerText = search_content.value;
    let w = measure.value.offsetWidth;
    if (w > 730) {
        main_input.value.style.width = '730px';
        return;
    }
    if (w > 150) {
        main_input.value.style.width = (w + 15) + 'px';
    }
}

function fun_click_space() {
    main_input.value.focus()
}

function fun_open_setting() {
    is_show_setting.value = !is_show_setting.value;
    win_set_size(600);
}
function fun_ompositionstart() {
    is_ompositioning = true;
}
function fun_ompositionend() {
    is_ompositioning = false;
    apps_menu.value.search();
}

</script>

<style scoped lang="less">
.MainPanel {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    position: relative;
    background-color: #252525;

    .InputArea {
        background-color: #252525;
        height: 50px;
        min-height: 50px;
        overflow: hidden;
        width: 100%;
        display: flex;
        justify-content: space-between;
        position: relative;

        .search {
            width: 160px;
            font-family: 'Arial', sans-serif;
            height: 50px;
            border-radius: 5px;
            font-size: 25px;
            outline: none;
            border: none;
            background-color: #252525;
            color: #f1f2f3;
            padding: 0 10px;
            user-select: none;
        }

        .space {
            flex-grow: 1;
        }

        .Icon {
            width: 50px;
            height: 50px;
            display: flex;
            align-items: center;
            user-select: none;
            cursor: pointer;
            will-change: filter;
            transition: 0.75s;

            img {
                width: 40px;
                height: 40px;
                border-radius: 20px;
            }

            &:hover {
                filter: drop-shadow(0 0 2em #eee);
            }
        }

        .block {
            position: absolute;
            font-size: 25px;
            color: #ccc;
            border: 1px dashed #eee;
            left: 10px;
            top: 0px;
        }
    }

    .AppMenu,
    .Setting {
        height: 120px;
        flex-grow: 1;
    }

    .measure {
        font-family: 'Arial', sans-serif;
        font-size: 25px;
        white-space: nowrap;
        padding: 0 10px;
        position: absolute;
        top: -200px;
    }

}
</style>