import { fs, invoke } from "@tauri-apps/api";

namespace FS {
    /**
     * @description 以utf-8编码读取文本所有内容并返回
     * @param filepath 文件路径
     * @returns 读取到的文本内容，如果文件不存在，则返回空字符串
     */
    export async function read_text(filepath: string) {
        if (await fs.exists(filepath)) {
            return fs.readTextFile(filepath);
        } else {
            return '';
        }
    }
    /**
     * 
     * @param filepath 文件路径
     * @param content 要写入文本的内容
     */
    export function write_text(filepath: string, content: string) {
        fs.writeTextFile(filepath, content);
    }
    /**
     * 
     * @param path 路径
     * @returns 路径是否存在
     */
    export function exists(path: string) {
        return fs.exists(path);
    }
    /**
     * 
     * @param path 本地文件路径
     * @returns 是否为文件夹
     */
    export function is_dir(path: string) {
        return invoke<boolean>('is_dir', { path: path });
    }

    /**
     * 
     * @param path 路径
     * @returns 获得无后缀名的文件名
     */
    export function file_stem(path: string) {
        let p = path.replace(/\\/g, '/');
        let filename = p.substring(p.lastIndexOf('/') + 1);
        let pos = filename.lastIndexOf('.');
        if (pos == -1) return filename;
        return filename.substring(0, pos);
    }
    /**
     * 
     * @param path 路径
     * @returns 获得完整的文件名
     */
    export function file_name(path: string) {
        let p = path.replace(/\\/g, '/');
        let filename = p.substring(p.lastIndexOf('/') + 1);
        return filename;
    }
    /**
     * 
     * @param path 路径
     * @returns 获取路径的父目录
     */
    export function parent(path: string) {
        let p = path.replace(/\\/g, '/');
        let parent = path.substring(0, p.lastIndexOf('/'));
        return parent;
    }
    /**
     * 
     * @param path 路径
     * @returns 获取文件路径扩展名
     */
    export function extension(path: string) {
        return path.substring(path.lastIndexOf('.') + 1);
    }
}

export default FS;