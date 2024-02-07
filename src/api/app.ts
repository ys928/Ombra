import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import Path from "./path";
import Dir from "./dir";
namespace App {

    /**
     * @description 获取当前系统所有已安装的app
     * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
     */
    export async function get_sys() {
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
     * @returns 判断当前app是否处于嵌入模式
     */
    export function is_embed() {
        return appWindow.label == 'MainWindow';
    }
    /**
     * 
     * @param target 目标文件路径
     * @returns 成功则返回其icon图片路径，失败返回空字符串
     */
    export async function get_icon(target: string) {
        let filename = Path.file_stem(target);
        let c = await Dir.config();
        let icon_path = await Path.join(c, 'icons', filename);
        let ret = await Path.exists(icon_path);
        if (ret) return icon_path;
        ret = await invoke<boolean>('get_associated_icon', { filePath: target, savePath: icon_path });
        if (ret) {
            return icon_path;
        } else {
            return '';
        }
    }
}

export default App;