import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

type SysAppInfo = {
    name: string,
    icon: string,
    start: string,
}

export default class App {

    /**
     * @description 获取当前系统所有已安装的app
     * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
     */
    static async get_sys_app(reload = false) {
        let app_sys_list = localStorage.getItem('app_sys_list');
        if (app_sys_list == null || reload) {
            let unlisten: UnlistenFn | undefined;
            const get_apps_result = new Promise<Array<SysAppInfo>>(async (resolve) => {
                //获取所有遍历到的程序
                unlisten = await listen<Array<SysAppInfo>>('get_all_app_result', (e) => {
                    resolve(e.payload)
                });
            });
            invoke('get_all_app');
            let apps_sys = await get_apps_result;
            if (unlisten) {
                unlisten();
            }
            let str_apps_sys = JSON.stringify(apps_sys);
            localStorage.setItem('app_sys_list', str_apps_sys);
            return apps_sys;
        } else {
            return JSON.parse(app_sys_list) as Array<SysAppInfo>;
        }
    }

    /**
     * 
     * @returns 判断当前app是否处于嵌入模式
     */
    static is_embed() {
        return appWindow.label == 'MainWindow';
    }
}