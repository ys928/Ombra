import { invoke, path } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
type FileInfo = {
    name: string,
    path: string,
    time: number,
    ftype: number, //1：文件、2：目录
}

export default class Directory {

    /**
     * 
     * @returns 返回当前电脑的配置目录
     */
    static config() {
        return path.appConfigDir();
    }

    /**
     * 
     * @param path 要遍历的目录
     * @param level 要遍历的层级，默认为1层，为0则递归遍历目录下的所有文件
     * @returns 返回文件信息列表
     */
    static async walk(path: string, level = 1) {
        let result = new Promise<Array<FileInfo>>((resolve) => {
            listen<Array<FileInfo>>('walk_dir_result', (e) => {
                resolve(e.payload);
            })
        });
        invoke('walk_dir', {
            path: path,
            level: level
        });
        return result;
    }
}