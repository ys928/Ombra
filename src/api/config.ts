import Directory from "./dir";
import Path from "./path";
import File from "./file";
import { WebUrlApp } from "./app";
export interface LocalApp {
    name: string;
    path: string; //路径作为id
    icon: string;
}

const recommand_urls: Array<WebUrlApp> = [
    {
        name: '必应',
        url: 'https://www.bing.com/search?q={query}',
        id: 'https://www.bing.com/search?q={query}',
        icon: '',
        on: true,
        features: ['text']
    }
]

async function read_config() {
    let p = await Config.path();
    let config = await File.read_text(p);
    let js_config = JSON.parse(config);
    return js_config;
}

async function write_config(config: any) {
    let p = await Config.path();
    let str = JSON.stringify(config);
    File.write_text(p, str);
}
async function read_item(item: string) {
    let cfg = await read_config();
    return cfg[item];
}

async function write_item(item: string, cnt: any) {
    let cfg = await read_config();;
    cfg[item] = cnt;
    write_config(cfg);
}
export default class Config {
    /**
     * 
     * @returns 获取配置文件路径
     */
    static async path() {
        let p = await Directory.config();
        p += 'config.json';
        if (!await Path.exists(p)) {
            File.write_text(p, '{}');
        }
        return p;
    }

    static async read_web_apps() {
        let ret = await read_item('web_apps') as Array<WebUrlApp> | undefined;
        if (ret == undefined) {
            write_item('web_apps', recommand_urls);
            ret = recommand_urls;
        }
        return ret;
    }

    static async write_web_apps(cnt: Array<WebUrlApp>) {
        write_item('web_apps', cnt);
    }

    static async read_local_app() {
        let ret = await read_item('local_apps') as Array<LocalApp> | undefined;
        if (ret == undefined) {
            write_item('local_apps', []);
            ret = [];
        }
        return ret;
    }

    static async write_local_app(cnt: Array<LocalApp>) {
        write_item('local_apps', cnt);
    }

    static async read_callout() {
        let ret = await read_item('callout') as string | undefined;
        if (ret == undefined) {
            write_item('callout', 'CommandOrControl+Shift+A');
            ret = "CommandOrControl+Shift+A";
        }
        return ret;
    }
    static async write_callout(callout: string) {
        write_item('callout', callout);
    }

    static async read_placeholder() {
        let ret = await read_item('placeholder') as string | undefined;
        if (ret == undefined) {
            write_item('placeholder', 'Hi，Ombra！');
            ret = "Hi，Ombra！";
        }
        return ret;
    }

    static async write_placeholder(placeholder: string) {
        write_item('placeholder', placeholder);
    }

    static async read_appinfo() {
        let ret = await read_item('appinfo') as Array<{ name: string, weight: number }> | undefined;
        if (ret == undefined) {
            write_item('appinfo', []);
            ret = [];
        }
        return ret;
    }

    static async write_appinfo(appinfo: Array<{ name: string, weight: number }>) {
        write_item('appinfo', appinfo);
    }
}