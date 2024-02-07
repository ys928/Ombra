import { invoke, path } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import Path from "./path";
namespace Dir {

    /**
     * 
     * @returns 返回当前电脑的配置目录
     */
    export function config() {
        return path.appConfigDir();
    }
    /**
     * 
     * @returns 返回存储图片的配置目录
     */
    export async function config_icon() {
        let ret = await config();
        return Path.join(ret, 'icons');
    }
    /**
     * 
     * @param path 要遍历的目录
     * @param level 要遍历的层级，默认为1层，为0则递归遍历目录下的所有文件
     * @returns 返回文件信息列表
     */
    export async function walk(path: string, level = 1) {
        let unlisten: UnlistenFn | undefined;
        let result = new Promise<Array<FileInfo>>(async (resolve) => {
            unlisten = await listen<Array<FileInfo>>('walk_dir_result', (e) => {
                resolve(e.payload);
            })
        });
        invoke('walk_dir', {
            path: path,
            level: level
        });
        let ret = await result;
        if (unlisten) unlisten();
        return ret;
    }
}

export default Dir;