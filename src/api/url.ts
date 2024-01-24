import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";

export default class Url {
    /**
     * @description 用系统默认浏览器打开链接
     * @param url 网络链接
     */
    static open(url: string) {
        invoke('open_web_url', { url: url });
    }
    /**
     * @description tauri不支持直接在窗口加载文件，比如图片，若要在窗口中加载图片之类的本地文件，需要使用该函数进行路径转换
     * @param filepath 电脑文件路径
     * @returns 返回可被窗口加载的路径
     */
    static convert(filepath: string) {
        return convertFileSrc(filepath);
    }
}