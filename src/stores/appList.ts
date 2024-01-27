import { defineStore } from "pinia";
import { Component, markRaw, reactive } from "vue";
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

    function add(name: string, id: string, icon: string, feature: Array<string>, self: boolean, component: Component | string | null, setup_callback: () => void, only_feature = false) {
        applist.push({
            name: name,
            icon: icon,
            feature: feature,
            only_feature: only_feature,
            weight: 0,
            id: id,
            component: component == null || typeof component == 'string' ? component : markRaw(component),
            self: self,
            setup: setup_callback
        });
    }
    return { applist, add };
});