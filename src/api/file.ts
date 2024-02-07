import { fs } from "@tauri-apps/api";

namespace File {
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
     * @param path 文件路径
     * @returns 文件是否存在
     */
    export function exists(path: string) {
        return fs.exists(path);
    }
}

export default File;