import { invoke } from "@tauri-apps/api";

namespace Explorer {

    /**
     * @description 用电脑默认应用打开该文件
     * @param path 文件路径
     */
    export function open_file(path: string) {
        invoke('default_open_file', { path: path });
    }
    /**
     * @description 打开资源管理器选择到指定文件
     * @param path 文件路径
     */
    export function select_file(path: string) {
        invoke('explorer_select_path', { path: path });
    }

    /**
     * 
     * @returns 返回当前聚焦的explorer窗口显示的路径
     */
    export function get_path() {
        return invoke<string>('get_explorer_show_path')
    }
}

export default Explorer;