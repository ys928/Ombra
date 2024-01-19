import { fs } from "@tauri-apps/api";

export default class File {
    /**
     * @description 以utf-8编码读取文本所有内容并返回
     * @param filepath 文件路径
     * @returns 读取到的文本内容
     */
    static read_text(filepath: string) {
        return fs.readTextFile(filepath);
    }
    /**
     * 
     * @param filepath 文件路径
     * @param content 要写入文本的内容
     */
    static write_text(filepath: string, content: string) {
        fs.writeTextFile(filepath, content);
    }
    /**
     * 
     * @param path 文件路径
     * @returns 文件是否存在
     */
    static exists(path: string) {
        return fs.exists(path);
    }
}