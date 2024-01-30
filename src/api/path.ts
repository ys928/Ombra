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
    /**
     * 
     * @param path 路径
     * @returns 获得无后缀名的文件名
     */
    static file_stem(path: string) {
        let p = path.replace(/\\/g, '/');
        let filename = p.substring(p.lastIndexOf('/')+1);
        let pos = filename.lastIndexOf('.');
        if (pos == -1) return filename;
        return filename.substring(0, pos);
    }
}