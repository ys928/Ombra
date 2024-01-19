<template>
    <div class="FileSearch">
        <AppTitlebar :id="app_file_search.id" :text="search_content"></AppTitlebar>
        <Search ref="vue_search" @fun_search="fun_search" @fun_exit="fun_exit"></Search>
        <Result ref="vue_result" :last_cnt="last_search.name" :last_mode="last_search.mode" :last_ext="last_search.ext"
            @fun_set_pop_menu="fun_set_pop_menu" @fun_search="fun_search" @fun_complete_search="is_searching = false">
        </Result>
        <Statusbar @fun_begin_idx="fun_begin_idx" @fun_search="fun_search" @fun_process="fun_process">
        </Statusbar>
        <PopMenu ref="div_pop_menu" @hidden="pop_menu.is_show = false" :isdir="click_item.isdir" :name="click_item.name"
            :path="click_item.path" :ext="click_item.ext" :x="pop_menu.x" :y="pop_menu.y" v-if="pop_menu.is_show">
        </PopMenu>
        <KLoading v-if="is_searching || is_processing"></KLoading>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { Ref, reactive, ref, watch } from 'vue';
import AppTitlebar from '~/components/AppTitlebar.vue'
import app_file_search from './App'
import PopMenu from './unit/PopMenu.vue'
import Search from './unit/Search.vue';
import Statusbar from './unit/Statusbar.vue';
import Result from './unit/Result.vue';
import { KLoading } from '~/kui';
import { win_close } from '~/ombra';
type FileInfo = {
    name: string,
    ext: string,
    path: string,
    time: number,
    isdir: boolean,
}

const search_content = ref("");
const div_pop_menu = ref() as Ref<HTMLElement>
//当前是否处于缓存数据状态
const is_processing = ref(false);

//绑定搜索组件
const vue_search = ref();
//绑定结果组件
const vue_result = ref();

//最后一次搜索状态，用于滚动事件重复发送
const last_search = reactive({
    name: '',
    ext: '',
    mode: 'normal'
});

//弹出菜单位置
const pop_menu = reactive({
    is_show: false,
    x: 0, y: 0
});

//滑动计数，-1代表已经达到底部
let scroll_count = 0;

//记录当前是否正处于搜索状态
let is_searching = ref(false);

let searching_task = {
    name: '',
    ext: '',
    mode: 'normal',
    deal: false
};
watch(is_searching, () => {
    if (is_searching.value) return;
    if (searching_task.deal == false) return;
    fun_search(searching_task.name, searching_task.ext, searching_task.mode, 0);
    searching_task.deal = false;
});


//创建索引缓存文件任务
function fun_begin_idx() {
    if (is_processing.value) {
        return;
    }
    is_processing.value = true;
    invoke('walk_all_files');
}

let click_item: FileInfo;
function fun_set_pop_menu(x: number, y: number, item: FileInfo) {
    pop_menu.x = x;
    pop_menu.y = y;
    click_item = item;
    pop_menu.is_show = true;
}

function fun_search(name: string, ext: string, mode: string, offset: number) {
    if (is_processing.value) return; //缓存状态禁止搜索

    if (offset == 0) { //为首次搜索
        scroll_count == 0;
        vue_result.value.clear_result();
        last_search.name = name;
        last_search.mode = mode;
        last_search.ext = ext;
    }

    if (is_searching.value) { //正处于搜索状态，将本次搜索任务进行缓存
        searching_task.deal = true;
        searching_task.name = name;
        searching_task.mode = mode;
        searching_task.ext = ext;
        return;
    }
    is_searching.value = true;

    const searchContent = {
        name: name,
        ext: ext,
        mode: mode,
        limit: 50,
        offset: offset,
    };
    invoke('search_file', searchContent);
}

function fun_exit() {
    win_close();
}

function fun_process(status: boolean) {
    is_searching.value = status;
}

</script>
  
<style scoped lang="less">
.FileSearch {
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    background-color: #191919;

    .KLoading {
        width: 50px;
        height: 50px;
        position: fixed;
        left: 50%;
        top: 50%;
        transform: translate(-50%, -50%);
    }
}
</style>