import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { Component, reactive } from "vue";
import { load_apps } from "~/apps/apps";
import Window from "./window";
export interface AppInfo {
    name: string, //应用名，将显示在应用菜单面板上
    id: string, //应用id，如果self属性为true，那么该选项必填，否则可选，将被用于独立窗口label、以及vue中的路由
    self: boolean, //是否需要独立窗口功能
    icon: string,   //图标所在路径
    feature: Array<string>, //该应用可处理的特点，当用户输出指定特点的内容时将被显示在推荐页，目前支持：filename、url、dir_path、file_path,explorer
    only_feature: boolean, //是否只在触发特点时才显示
    weight: number, //应用权重，随用户使用次数进行计算
    component: Component | string | null, //如果是有界面的内置应用，需要填入组件，如果为外部的插件应用，则需要填入index.html文件位置，如果都不是，则填入null
    setup: () => void, //当用户点击该app时将被立即执行的函数
}
type SysAppInfo = {
    name: string,
    icon: string,
    start: string,
}

//所有可用的应用列表
const app_list = reactive([]) as Array<AppInfo>
export default class App {

    /**
     * @description 获取当前系统所有已安装的app
     * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
     */
    static async get_sys_app(reload = false) {
        let app_sys_list = localStorage.getItem('app_sys_list');
        if (app_sys_list == null || reload) {
            const get_apps_result = new Promise<Array<SysAppInfo>>((resolve) => {
                //获取所有遍历到的程序
                listen<Array<SysAppInfo>>('get_all_app_result', (e) => {
                    resolve(e.payload)
                });
            });
            invoke('get_all_app');
            let apps_sys = await get_apps_result;
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

    static async get_applist() {
        if (app_list.length == 0) {
            await load_apps();
        }
        return app_list;
    }

    /**
     * @description 添加app应用
     * @param name 
     * @param id 
     * @param icon 
     * @param feature 
     * @param self 
     * @param setup_callback 
     * @param only_feature 
     * @param plugin_index 
     */
    static add(name: string, id: string, icon: string, feature: Array<string>, self: boolean, component: Component | string | null, setup_callback: Function, only_feature = false) {
        app_list.push({
            name: name,
            icon: icon,
            feature: feature,
            only_feature: only_feature,
            weight: 0,
            id: id,
            component: component,
            self: self,
            setup: () => {
                if (self) {
                    Window.set_size(600);
                    Window.set_resizable(true);
                } else {
                    Window.hide();
                }
                setup_callback();
            }
        });
    }
}