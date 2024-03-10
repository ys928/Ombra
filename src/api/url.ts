import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
//@ts-ignore
import parseUrl from 'parse-url'
import Dir from "./dir";
import Path from "./path";
namespace Url {
    /**
     * @description 用系统默认浏览器打开链接
     * @param url 网络链接
     */
    export function open(url: string) {
        invoke('open_web_url', { url: url });
    }
    /**
     * @description tauri不支持直接在窗口加载文件，比如图片，若要在窗口中加载图片之类的本地文件，需要使用该函数进行路径转换
     * @param filepath 电脑文件路径
     * @returns 返回可被窗口加载的路径
     */
    export function convert(filepath: string) {
        return convertFileSrc(filepath);
    }

    export function domain(url: string) {
        let ret = parseUrl(url);
        return ret.protocol + "://" + ret.host;
    }

    export async function download(url: string, file: string) {
        let ret = await invoke<boolean>('download_file', { url: url, file: file });
        return ret;
    }

    export async function download_favicon(url: string, name: string) {
        let icon_path = await Dir.config_icon();
        let fav_file = await Path.join(icon_path, name);
        let ret = await invoke<boolean>('web_icon_to_file', { url: url, savePath: fav_file });
        if (ret) {
            return fav_file;
        }
        return '';
    }
}

export default Url;