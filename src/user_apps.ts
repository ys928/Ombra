import AppFileSearch from "./apps/FileSearch/App";
import AppOpenLink from "./apps/OpenLink/App"
import AppOpenPath from "./apps/OpenPath/App"
import AppOpenFile from "./apps/OpenFile/App"
import PluginWindow from './components/PluginWindow.vue'
import PluginDevelopTool from './apps/PluginDevelopTool/App'
import { add_app } from "./global";
import { dir_walk, file_convert } from '~/ombra'
import { path } from "@tauri-apps/api";
import File from '~/api/file'
import Window from "./api/window";
let user_apps_list = [
    AppFileSearch,
    AppOpenLink,
    AppOpenPath,
    AppOpenFile,
    PluginDevelopTool
]

let app_route = [] as Array<{
    path: string,
    component: any
}>


export async function load_user_app() {
    let is_init = localStorage.getItem('is_init');
    //加载内嵌应用
    for (let app of user_apps_list) {
        let url = '/apps/' + app.id;
        //仅限主窗口执行加载app代码
        if (Window.is_main()) {
            if (is_init != 'true') {
                //@ts-ignore
                if (app.preload) {
                    //@ts-ignore
                    app.preload();
                }
            }
            add_app(app.name, app.id, app.icon, app.feature, app.self, app.setup, app.only_feature);
        }
        if (app.self == false || app.component == undefined || app.id.length == 0 || app.component == null) {
            continue
        }
        app_route.push({
            path: url,
            component: app.component
        });
    }
    //加载插件应用
    await load_plugin_app();
    localStorage.setItem('is_init', 'true');
    return app_route;
};

async function load_plugin_app() {
    let plugin_path = await path.resolve('./plugin');
    let plugin_dirs = await dir_walk(plugin_path);
    for (let i = 0; i < plugin_dirs.length; i++) {
        let name = '';
        let icon = '';
        let plugin_index = '';
        let id = ''
        let features = [];
        let plugin_files = await dir_walk(plugin_dirs[i].path + '\\' + plugin_dirs[i].name);
        for (let f of plugin_files) {
            if (f.name.startsWith('icon')) { //icon图标
                icon = file_convert(f.path + '\\' + f.name);
            } else if (f.name == 'index.html') {
                plugin_index = file_convert(f.path + '\\' + f.name);
            } else if (f.name == 'config.json') {
                let text = await File.read_text(f.path + '\\' + f.name);
                let config = JSON.parse(text);
                features = config.features;
                id = config.id;
                name = config.name;
            }
        }
        if (Window.is_main()) {
            add_app(name, id, icon, features, true, () => { }, false, plugin_index);
        }
        app_route.push({
            path: '/apps/' + id,
            component: PluginWindow
        });
    }
}