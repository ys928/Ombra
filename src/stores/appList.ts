import { defineStore } from "pinia";
import { Component, markRaw, reactive } from "vue";
import { WebUrlApp } from "~/api/app";
import Ombra from "~/api/ombra";
import Url from "~/api/url";
export interface AppInfo {
    name: string, //应用名，将显示在应用菜单面板上
    id: string, //应用id，如果component属性为不为null，那么该选项必填，否则可选
    icon: string,   //图标所在路径
    feature: Array<string>, //该应用可处理的特点，当用户输出指定特点的内容时将被显示在推荐页，目前支持：filename、url、dir_path、file_path,explorer
    only_feature: boolean, //是否只在触发特点时才显示
    weight: number, //应用权重，随用户使用次数进行计算
    component: Component | string | null, //带界面的内置应用需填入组件，外部的插件应用需要填入index.html文件位置，如果都不是，则应填入null代表无界面应用
    setup: () => void, //当用户点击该app时将被立即执行的函数
}

export const useAppListStore = defineStore('applist', () => {
    const applist = reactive([]) as Array<AppInfo>;
    /**
    * @description 添加app应用
    * @param name //应用名，将显示在应用菜单面板上
    * @param id //应用id，如果self属性为true，那么该选项必填，否则可选，将被用于独立窗口label、以及vue中的路由
    * @param icon //图标所在路径
    * @param feature //该应用可处理的特点，当用户输出指定特点的内容时将被显示在推荐页，目前支持：filename、url、dir_path、file_path,explorer，text
    * @param self //是否需要独立窗口功能
    * @param component //如果是有界面的内置应用，需要填入组件，如果为外部的插件应用，则需要填入index.html文件位置，如果都不是，则填入null
    * @param setup_callback  //当用户点击该app时将被立即执行的函数
    * @param only_feature //是否只在触发特点时才显示
    */

    function add(name: string, id: string, icon: string, feature: Array<string>, component: Component | string | null, setup_callback: () => void, only_feature = false) {
        if (id.length != 0) {
            for (let app of applist) {
                if (app.id == id) return 'repeat'; //不允许重复添加id相同的app
            }
        }

        applist.push({
            name: name,
            icon: icon,
            feature: feature,
            only_feature: only_feature,
            weight: 0,
            id: id,
            component: component == null || typeof component == 'string' ? component : markRaw(component),
            setup: setup_callback
        });
        return 'success';
    }

    function add_web(app: WebUrlApp) {
        add(app.name, app.id, app.icon, app.features, null, () => {
            let features = Ombra.get_features();
            let text = Ombra.get_text();
            let u;
            if (features.includes('text')) {
                u = app.url.replace('{query}', text);
            } else {
                u = app.url.replace('{query}', '');
            }
            Url.open(u);
        });
    }

    function remove(id: string) {
        for (let i = 0; i < applist.length; i++) {
            if (applist[i].id == id) {
                applist.splice(i, 1);
                break;
            }
        }
    }

    return { applist, add, add_web, remove };
});