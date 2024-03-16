import { invoke } from "@tauri-apps/api";
import { defineStore } from "pinia";
import { reactive, ref, watch } from "vue";
let searching_task = {
    name: '',
    ext: '',
    mode: 'normal',
    deal: false
};
export const useSearchResultStore = defineStore('file_search_result', () => {
    const result = reactive([]) as Array<FileInfo>;
    const is_processing = ref(false);
    const is_searching = ref(false);
    //最后一次搜索状态，用于滚动事件重复发送
    let last_search = {
        name: '',
        ext: '',
        mode: 'normal'
    };
    //滑动计数，-1代表已经达到底部
    let scroll_count = ref(0);

    watch(is_searching, () => {
        if (is_searching.value) return;
        if (searching_task.deal == false) return;
        search(searching_task.name, searching_task.ext, searching_task.mode, 0);
        searching_task.deal = false;
    });

    async function search(name: string, ext: string, mode: string, offset: number) {
        if (is_processing.value) return; //缓存状态禁止搜索
        console.log("search", offset);
        if (offset == 0) { //为首次搜索
            scroll_count.value = 0;
            result.length = 0;
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
        let ret = await invoke<Array<FileInfo>>('search_file', searchContent);
        console.log(ret.length);
        if (ret.length < 50) scroll_count.value = -1;// 到底部了
        result.push(...ret);
        is_searching.value = false;
    }

    function set_process_status(status: boolean) {
        is_processing.value = status;
    }

    function set_search_status(status: boolean) {
        is_searching.value = status;
    }
    function set_scroll_count(count: number) {
        scroll_count.value = count;
    }

    function remove(path: string, name: string, ext: string) {
        for (let i = 0; i < result.length; i++) {
            if (result[i].path == path && result[i].name == name && result[i].ext == ext) {
                result.splice(i, 1);
                break;
            }
        }
    }

    return { result, is_processing, last_search, is_searching, remove, search, set_process_status, set_search_status, set_scroll_count, scroll_count };
});