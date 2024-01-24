import { globalShortcut } from "@tauri-apps/api";

export default class GlobalShortcut {
    /**
     * 
     * @param shortkey 要注册的全局快捷键
     * @param callback 相应的回调函数
     */
    static register(shortkey: string, callback: Function) {
        globalShortcut.register(shortkey, () => {
            callback();
        });
    }
    /**
     * 
     * @param shortkey 取消注册的全局快捷键
     */
    static unregister(shortkey: string) {
        globalShortcut.unregister(shortkey);
    }
    /**
     * 
     * @param shortkey 全局快捷键
     * @returns 是否已经被注册
     */
    static is_registered(shortkey: string) {
        return globalShortcut.isRegistered(shortkey);
    }
}