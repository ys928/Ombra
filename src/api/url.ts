import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/tauri";
//@ts-ignore
import parseUrl from 'parse-url'
import Dir from "./dir";
import Path from "./path";
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

    static domain(url: string) {
        let ret = parseUrl(url);
        return ret.protocol + "://" + ret.host;
    }

    static async download(url: string, file: string) {
        let unlisten: UnlistenFn | undefined;
        const download_result = new Promise<boolean>(async (resolve) => {
            //获取下载结果
            unlisten = await listen<boolean>('download_file_result', (e) => {
                resolve(e.payload)
            });
        });
        invoke('download_file', { url: url, file: file });
        let is_success = await download_result;
        if (unlisten) {
            unlisten();
        }
        return is_success;
    }

    static async download_favicon(url: string, name: string) {
        let domain = Url.domain(url);
        let favicon = domain + '/favicon.ico';
        let icon_path = await Dir.config_icon();
        let fav_file = await Path.join(icon_path, name);
        let ret = await this.download(favicon, fav_file);
        if (ret) {
            return fav_file;
        }
        return '';
    }
}