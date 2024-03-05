import { invoke, path } from "@tauri-apps/api";
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
     * @returns 返回文件信息列表
     */
    export async function walk(path: string) {
        return await invoke<Array<FileInfo>>('walk_dir', {
            path: path
        });
    }
}

export default Dir;