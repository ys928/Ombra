import FileSearchVue from "./FileSearch.vue"
import { invoke } from "@tauri-apps/api"
import { add_timing_task, read_config_item, write_config_item } from "~/global"
//导出app注册信息对象
export default {
    name: '文件搜索',
    id: 'file-search',
    icon: '/imgs/file_search.jpg',
    feature: ['filename'],
    only_feature: false,
    component: FileSearchVue,
    self: true,
    setup: async () => {
        let ret = await read_config_item('walk_files_time_interval');
        if (ret == undefined) {
            ret = 60 * 24;
            write_config_item('walk_files_time_interval', ret);
        }
        add_timing_task(Number(ret), () => {
            invoke('walk_all_files');
        }
        )
    }
}