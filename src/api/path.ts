import { fs, invoke, path } from "@tauri-apps/api";

export default class Path {
    /**
     * @description 判断路径是文件还是目录
     * @param path 要判断文件的路径
     * @returns dir、file、or error
     */
    static async judge(path: string) {
        return await invoke<string>('dir_or_file', { path: path });
    }

    /**
     * 
     * @param path 路径
     * @returns 返回该路径是否存在
     */
    static async exists(path: string) {
        return fs.exists(path);
    }

    /**
     * 
     * @param paths 多个路径
     * @returns 返回拼接好的路径
     */
    static join(...paths: string[]) {
        return path.join(...paths);
    }
}