<script setup lang="ts">
import { Ref, nextTick, onMounted, onUnmounted, ref } from "vue";
import AppMenu from "./MainPanel/AppMenu.vue";
import { onTextUpdate, readText, startListening } from "tauri-plugin-clipboard-api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import Window from "~/api/window";
import GlobalShortcut from "~/api/globalShortcut";
import Explorer from "~/api/explorer";
import Config from "~/api/config";

const main_input = ref() as Ref<HTMLInputElement>;
const measure = ref() as Ref<HTMLElement>;
const search_content = ref("");

const apps_menu = ref();
const cur_focus_app = ref(0);
const is_show_setting = ref(false);
const search_input_placeholder = ref('');
const callout_short_key = ref('');

//由于tauri中移动窗口也会相继触发blur、focus，因此使用该变量判断
let is_hide = false;

let fun_eve_blur: UnlistenFn | undefined;
let fun_eve_focus: UnlistenFn | undefined;

let fun_eve_click_tray: UnlistenFn | undefined;;

let unlisten_single_instance: UnlistenFn | undefined;

let unlistenTextUpdate: UnlistenFn;
let unlistenClipboard: () => Promise<void>;
let clip_board_time = 999;
let timing_fun: string | number | NodeJS.Timeout | undefined;
onMounted(async () => {
    main_input.value.focus();
    //读取唤出面板的快捷键
    callout_short_key.value = await Config.read_callout();
    set_callout_shortkey(callout_short_key.value);
    //读取搜索栏占位符
    let placeholder = await Config.read_placeholder();
    search_input_placeholder.value = placeholder;
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

    unlisten_single_instance = await listen('single-instance', async () => {
        if (!await Window.is_visible()) {
            Window.show();
        }
    })

    fun_eve_blur = await Window.event_blur('MainWindow', () => {
        is_hide = true;
        setTimeout(() => {
            if (is_hide) {
                Window.hide();
            }
        }, 100);
    })

    fun_eve_focus = await Window.event_focus('MainWindow', () => {
        is_hide = false;
    });

    fun_eve_click_tray = await Window.event_click_tray(fun_switch_panel_status);
});

onUnmounted(async () => {
    unlistenTextUpdate();
    unlistenClipboard();
    
    if (fun_eve_blur) fun_eve_blur();

    if (fun_eve_focus) fun_eve_focus();

    if (fun_eve_click_tray) fun_eve_click_tray();

    if (unlisten_single_instance) unlisten_single_instance();
})

async function set_callout_shortkey(shortkey: string) {
    GlobalShortcut.auto_set(shortkey, fun_switch_panel_status);
    Config.write_callout(shortkey);
    callout_short_key.value = shortkey;
}

//用于切换面板的显示、隐藏两种状态
async function fun_switch_panel_status() {
    // console.log("fun_callout_panel");
    if (await Window.is_visible()) {
        Window.hide();
        return;
    }
    let p = await Explorer.get_path();
    if (p == 'none') {
        apps_menu.value.init_feature([], '');
    } else {
        apps_menu.value.init_feature(['explorer'], p);
    }
    Window.show();
    Window.focus();
    if (clip_board_time >= 0 && clip_board_time <= 3) {
        search_content.value = await readText();
        fun_input();
    } else {
        main_input.value.select();
    }
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
            Window.hide();
        }
    }
}
let is_ompositioning = false;

let need_search = false;

let is_in_searching = false;

async function fun_input() {
    is_show_setting.value = false;
    if (is_ompositioning) return;
    if (is_in_searching) { //如果当前处于搜索状态，则缓存
        need_search = true;
        return;
    }
    //否则开始搜索
    is_in_searching = true;
    adjust_input_size();
    cur_focus_app.value = 0;
    await apps_menu.value.search();
    is_in_searching = false;
    if (need_search) { //搜索结束，如果需要搜索，则继续
        need_search = false;
        fun_input();
    }

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

function fun_ompositionstart() {
    is_ompositioning = true;
}
function fun_ompositionend() {
    is_ompositioning = false;
    apps_menu.value.search();
}

</script>

<template>
    <div class="MainPanel">
        <div class="InputArea">
            <input class="search" v-model="search_content" ref="main_input" @input="fun_input"
                @keydown="fun_keydown($event)" @compositionstart="fun_ompositionstart" @compositionend="fun_ompositionend"
                :placeholder="search_input_placeholder">
            <div class="space" @mousedown="main_input.focus()" @dblclick="fun_click_space" data-tauri-drag-region></div>
            <div class="Icon" @click="Window.to_setting">
                <img src="/logo.png" draggable="false">
            </div>
        </div>
        <AppMenu ref="apps_menu" :main_input="main_input" :search_content="search_content"
            v-model:cur_focus_app="cur_focus_app">
        </AppMenu>
        <div class="measure" ref="measure" style="visibility: hidden;"></div>
    </div>
</template>

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

            &::selection {
                background-color: #263C58;
            }
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