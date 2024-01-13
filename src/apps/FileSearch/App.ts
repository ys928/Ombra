import FileSearchVue from "./FileSearch.vue"
import { invoke } from "@tauri-apps/api/tauri";
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
    },
    preload: () => {
        //应用启动时自动进行索引
        invoke('walk_all_files');
    }
}