<template>
    <div class="StatusBar">
        <span class="progress"> {{ use_seconds }} {{ task_status }}</span>
        <span class="file_num">
            文件总数：<span> {{ file_num }} </span>
        </span>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { onMounted, onUnmounted, ref } from 'vue';
const emits = defineEmits(['fun_search', 'fun_process'])
type TaskProgress = {
    status: string, //状态
    data: string,   //传送数据
}
const file_num = ref(0);
const use_seconds = ref();
const task_status = ref();

let timerInterval: NodeJS.Timeout | undefined;
let unlisten: UnlistenFn | undefined;
onMounted(async () => {
    file_num.value = await invoke<number>('get_file_catch_info');

    timerInterval = setInterval(async () => {
        let num = await invoke<number>('get_file_catch_info');
        if (file_num.value != 0 && num == 0) {
            return;
        }
        file_num.value = num;
    }, 10 * 1000);

    //获取遍历进度
    unlisten = await listen<TaskProgress>('walk_files_process', async (e) => {
        // 初始化计时器变量
        if (e.payload.status == 'walking') { //正在遍历文件夹
            task_status.value = '正在索引文件：' + e.payload.data;
            emits('fun_process', true);
        } else if (e.payload.status == 'begin_save') {
            task_status.value = '正在缓存：' + e.payload.data;
        } else if (e.payload.status == 'end') {
            task_status.value = '已完成数据缓存';
            emits('fun_process', false);
            emits('fun_search', '', '', 'normal', 0);
        }
    })
});

onUnmounted(() => {
    clearInterval(timerInterval);
    if (unlisten) unlisten();
});


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