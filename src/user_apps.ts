import AppFileSearch from "./apps/FileSearch/App";
import AppOpenLink from "./apps/OpenLink/App"
import AppOpenPath from "./apps/OpenPath/App"
import AppOpenFile from "./apps/OpenFile/App"
import { add_app } from "./global";
import { win_is_main } from '~/ombra'
let user_apps_list = [
    AppFileSearch,
    AppOpenLink,
    AppOpenPath,
    AppOpenFile
]

let app_route = [] as Array<{
    path: string,
    component: any
}>


export function load_user_app() {
    for (let app of user_apps_list) {
        let url = '/apps/' + app.id;
        //仅限主窗口执行加载app代码
        if (win_is_main()) {
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
    return app_route;
};