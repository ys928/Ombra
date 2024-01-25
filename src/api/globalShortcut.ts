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
    /**
     * @description 自动注册全局快捷键，如果该快捷键已被注册，会尝试先将其取消注册、然后再注册
     * @param shortkey 要注册的快捷键
     * @param func 当快捷键按下后的回调函数
     */
    static async auto_set(shortkey: string, func: Function) {
        const isreg = await GlobalShortcut.is_registered(shortkey);
        if (isreg) {
            await GlobalShortcut.unregister(shortkey);
        }
        GlobalShortcut.register(shortkey, func);
    }
}