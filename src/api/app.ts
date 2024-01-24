import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

type AppInfo = {
    name: string,
    icon: string,
    start: string,
}
export default class App {

    /**
     * @description 获取当前系统所有已安装的app
     * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
     */
    static async get_all() {
        const get_apps_result = new Promise<Array<AppInfo>>((resolve) => {
            //获取所有遍历到的程序
            listen<Array<AppInfo>>('get_all_app_result', (e) => {
                resolve(e.payload)
            });
        });
        invoke('get_all_app');
        return await get_apps_result;
    }

    /**
     * 
     * @returns 判断当前app是否处于嵌入模式
     */
    static is_embed() {
        return appWindow.label == 'MainWindow';
    }
}