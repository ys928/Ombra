<template>
    <div class="StatusBar">
        <span class="progress"> {{ use_seconds }} {{ task_status }}</span>
        <span class="file_catch">
            <span v-if="file_catch.is_exist">
                更新于：{{ time_ago(file_catch.time) }}，文件总数：{{ file_catch.file_num }}
            </span>
            <span v-else>
                当前还没有缓存文件数据
            </span>
            <span class="index" @click="fun_idx">点击索引</span>
        </span>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, ref } from 'vue';
import { time_ago } from '~/global';
const emits = defineEmits(['fun_begin_idx', 'fun_search','fun_complete'])
type TaskProgress = {
    status: string, //状态
    data: string,   //传送数据
}
const file_catch = ref({
    is_exist: false,
    time: 0,
    file_num: 0
});
const use_seconds = ref();
const task_status = ref();

onMounted(async () => {
    file_catch.value = await invoke('get_file_catch_info');
});

onUnmounted(() => {
});

let secodes = 0;
function fun_idx() {
    // 创建并启动计时器，每秒更新一次
    secodes = 0;
    timerInterval = setInterval(function () {
        secodes++;
        use_seconds.value = secodes + 's';
    }, 1000);
    emits('fun_begin_idx');
}
let timerInterval: string | number | NodeJS.Timeout | undefined;
//获取遍历进度
listen<TaskProgress>('walk_files_process', async (e) => {
    // 初始化计时器变量
    if (e.payload.status == 'walking') { //正在遍历文件夹
        task_status.value = '正在索引文件：' + e.payload.data;
    } else if (e.payload.status == 'begin_save') {
        task_status.value = '正在缓存：' + e.payload.data;
    } else if (e.payload.status == 'end') {
        task_status.value = '已完成数据缓存';
        emits('fun_complete');
        clearInterval(timerInterval);
        emits('fun_search', '', 'normal', 0);
    }
})
</script>


<style scoped lang="less">
.StatusBar {
    height: 30px;
    color: antiquewhite;
    display: flex;
    justify-content: space-between;
    padding: 0 20px;
    border-top: 1px solid #2b2b2b;
    font-size: 12px;
    line-height: 30px;
    color: #aaa;
    user-select: none;

    .index {
        margin: 0 5px;
        padding: 3px 5px;
        border-radius: 5px;
        cursor: pointer;
        background-color: #2f2f2f;

        &:hover {
            background-color: #454545;
        }
    }
}
</style>