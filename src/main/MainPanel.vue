<script setup lang="ts">
import { Ref, onMounted, onUnmounted, ref } from "vue";
import AppMenu from "./MainPanel/AppMenu.vue";
import Search from "./MainPanel/Search.vue";
import { onTextUpdate, readText, startListening } from "tauri-plugin-clipboard-api";
import { listen } from "@tauri-apps/api/event";
import { Window, GlobalShortcut, Explorer, Ombra } from "~/api";
import { useConfigStore } from "~/stores/config";

const configStore = useConfigStore();

const main_input = ref();
const measure = ref() as Ref<HTMLElement>;

const apps_menu = ref();
const cur_focus_app = ref(0);
const is_show_setting = ref(false);
const search_input_placeholder = ref('');
const callout_short_key = ref('');

const search_content = ref("");

const block_content = ref('');

//由于tauri中移动窗口也会相继触发blur、focus，因此使用该变量判断
let is_hide = false;

let fun_eve_blur: UnlistenFn | undefined;
let fun_eve_focus: UnlistenFn | undefined;

let fun_eve_click_tray: UnlistenFn | undefined;;

let unlisten_single_instance: UnlistenFn | undefined;

let unlistenTextUpdate: UnlistenFn | undefined;
let unlistenClipboard: () => Promise<void>;
let clip_board_time = 999;
let timing_fun: string | number | NodeJS.Timeout | undefined;
onMounted(async () => {
    main_input.value.focus();
    fun_search();
    //读取唤出面板的快捷键
    callout_short_key.value = await configStore.read_callout();
    set_callout_shortkey(callout_short_key.value);
    //读取搜索栏占位符
    let placeholder = await configStore.read_placeholder();
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
            Window.focus();
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

    if (unlistenTextUpdate) unlistenTextUpdate();

    if (unlistenClipboard) unlistenClipboard();

    if (fun_eve_blur) fun_eve_blur();

    if (fun_eve_focus) fun_eve_focus();

    if (fun_eve_click_tray) fun_eve_click_tray();

    if (unlisten_single_instance) unlisten_single_instance();
})

async function set_callout_shortkey(shortkey: string) {
    GlobalShortcut.auto_set(shortkey, fun_switch_panel_status);
    configStore.write_callout(shortkey);
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
        Ombra.set_text('');
        Ombra.set_features([]);
        apps_menu.value.search(true);
    } else {
        Ombra.set_text(p);
        Ombra.set_features(['explorer']);
        apps_menu.value.search(true);
    }
    Window.show();
    Window.focus();
    if (clip_board_time <= 3) {
        let text = await readText();
        let text_trim = text.trim();
        Ombra.set_text(text_trim);
        if (text_trim.indexOf('\n') == -1) {
            search_content.value = text_trim;
        } else {
            if (text_trim.length > 50) {
                block_content.value = text_trim.substring(0, 50).replace('\n', '¬') + ' ......';
            } else {
                block_content.value = text_trim.replace('\n', '¬');
            }
            search_input_placeholder.value = "文本";
        }
        await apps_menu.value.search();
    } else {
        main_input.value.selected();
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
        let r = window.getSelection()?.getRangeAt(0);
        if (r && r.startOffset == 0) {
            apps_menu.value.move('left');
        }
        return;
    }

    if (e.key == 'ArrowRight') {
        let r = window.getSelection()?.getRangeAt(0);
        if (r && r.startOffset == search_content.value.length) {
            apps_menu.value.move('right');
        }
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
            fun_search();
        } else {
            Window.hide();
        }
        return;
    }
    if (e.key == 'Backspace') {
        if (search_content.value.length == 0 && block_content.value.length > 0) {
            block_content.value = ''
            //读取搜索栏占位符
            let placeholder = await configStore.read_placeholder();
            search_input_placeholder.value = placeholder;
            fun_search();
        }
    }
}

let need_search = false;

let is_in_searching = false;

async function fun_search() {
    is_show_setting.value = false;
    if (is_in_searching) { //如果当前处于搜索状态，则缓存
        need_search = true;
        return;
    }
    //否则开始搜索
    is_in_searching = true;
    cur_focus_app.value = 0;
    Ombra.set_text(search_content.value);
    await apps_menu.value.search();
    is_in_searching = false;
    if (need_search) { //搜索结束，如果需要搜索，则继续
        need_search = false;
        fun_search();
    }

}

function fun_click_space() {
    main_input.value.focus()
}

async function fun_paste(e: ClipboardEvent) {
    e.preventDefault();
    e.stopPropagation();
    let text = await readText();
    let text_trim = text.trim();
    Ombra.set_text(text_trim);
    if (text_trim.indexOf('\n') == -1) {
        search_content.value = text_trim;
    } else {
        if (text_trim.length > 50) {
            block_content.value = text_trim.substring(0, 50).replace('\n', '¬') + ' ......';
        } else {
            block_content.value = text_trim.replace('\n', '¬');
        }
        search_input_placeholder.value = "文本";
    }
    await apps_menu.value.search();
}

</script>

<template>
    <div class="MainPanel">
        <div class="InputArea">
            <div class="core">
                <div class="block" v-if="block_content.length > 0" v-text="block_content"></div>
                <Search ref="main_input" v-model:value="search_content" @keydown="fun_keydown" @change="fun_search"
                    @paste="fun_paste" :placeholder="search_input_placeholder">
                </Search>
                <div class="space" @mousedown="main_input.focus()" @dblclick="fun_click_space" data-tauri-drag-region>
                </div>
            </div>
            <div class="Icon" @click="Window.to_setting">
                <img src="/logo.png" draggable="false">
            </div>
        </div>
        <AppMenu ref="apps_menu" :main_input="main_input" v-model:cur_focus_app="cur_focus_app">
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

        .core {
            width: 100px;
            flex-grow: 1;
            height: 50px;
            display: flex;

            .block {
                color: #eee;
                margin: 10px 5px;
                padding: 0 10px;
                line-height: 30px;
                font-size: 14px;
                border: 1px dashed #999;
            }

            .space {
                flex-grow: 1;
            }
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