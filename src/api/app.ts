import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import Config from "./config";

type SysAppInfo = {
    name: string,
    icon: string,
    start: string,
}

export interface WebUrlApp {
    name: string,//网站名
    url: string,//网页路径
    id: string,//用网站域名作为id
    icon: string,//网页图标
    on: Boolean,//时候启动
    features: Array<string>, //可被匹配的特性
}

const recommand_urls: Array<WebUrlApp> = [
    {
        name: '必应',
        url: 'https://www.bing.com/search?q={query}',
        id: 'www.bing.com',
        icon: '/web/bing.ico',
        on: true,
        features: ['text']
    }
]
export default class App {

    /**
     * @description 获取当前系统所有已安装的app
     * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
     */
    static async get_sys() {
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
        return apps_sys;
    }
    /**
     * 
     * @returns 获取所有可用的web app
     */
    static async get_web() {
        let urls_app = await Config.read_web_apps();
        if (urls_app == undefined) {
            Config.write_web_apps(recommand_urls);
            return recommand_urls;
        }
        return urls_app;
    }

    /**
     * 
     * @returns 判断当前app是否处于嵌入模式
     */
    static is_embed() {
        return appWindow.label == 'MainWindow';
    }
}