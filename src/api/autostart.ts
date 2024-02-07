import { invoke } from "@tauri-apps/api";

namespace AutoStart {
    /**
     * 
     * @returns 当前应用时候已经自启动
     */
    export function is_set() {
        return invoke<boolean>('is_auto_start');
    }
    /**
     * 
     * @param start 开启或关闭自启动
     * @returns 是否设置成功
     */
    export function set(start: boolean) {
        return invoke<boolean>('auto_start', { start: start });
    }
}

export default AutoStart;