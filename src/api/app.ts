import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { FS, Path, Dir } from '.'
namespace App {

    /**
     * @description 获取当前系统所有已安装的app
     * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
     */
    export async function get_sys() {
        return await invoke<Array<SysAppInfo>>('get_all_app');
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
        let filename = FS.file_stem(target);
        let c = await Dir.config();
        let icon_path = await Path.join(c, 'icons', filename);
        let ret = await FS.exists(icon_path);
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