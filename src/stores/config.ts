import { defineStore } from "pinia";
import { Dir, Path, File } from "~/api";

interface CfgAppInfo {
    name: string,
    weight: number
}
interface Config {
    callout: string,
    placeholder: string,
    local_apps: Array<LocalApp>,
    appinfo: Array<CfgAppInfo>,
    web_apps: Array<WebUrlApp>,
}


const default_config: Config = {
    callout: 'CommandOrControl+Shift+A',
    placeholder: 'Hi，Ombra！',
    local_apps: [],
    appinfo: [],
    web_apps: [{
        name: '必应',
        url: 'https://www.bing.com/search?q={query}',
        icon: '',
        on: true,
        features: ['text']
    },
    {
        name: '谷歌',
        url: 'https://www.google.com/search?q={query}',
        icon: '',
        on: true,
        features: ['text']
    }]
}

export const useConfigStore = defineStore('config', () => {
    let config = {} as Config;

    async function init() {
        let p = await Dir.config();
        p = await Path.join(p, 'config.json');
        if (!await Path.exists(p)) {
            File.write_text(p, JSON.stringify(default_config));
            config = default_config;
        } else {
            let text = await File.read_text(p);
            config = JSON.parse(text);
        }
    }
    async function write_config() {
        let p = await Dir.config();
        p = await Path.join(p, 'config.json');
        File.write_text(p, JSON.stringify(config));
    }


    async function read_web_apps() {
        let web_apps = config['web_apps'];
        if (web_apps == undefined) {
            config['web_apps'] = [];
            write_config();
        }
        return config['web_apps'];
    }

    async function write_web_apps(cnt: Array<WebUrlApp>) {
        config['web_apps'] = cnt;
        write_config();
    }

    async function read_local_app() {
        let web_apps = config['local_apps'];
        if (web_apps == undefined) {
            config['local_apps'] = [];
            write_config();
        }
        return config['local_apps'];
    }

    async function write_local_app(cnt: Array<LocalApp>) {
        config['local_apps'] = cnt;
        write_config();
    }

    async function read_callout() {
        let web_apps = config['callout'];
        if (web_apps == undefined) {
            config['callout'] = "CommandOrControl+Shift+A";
            write_config();
        }
        return config['callout'];
    }
    async function write_callout(callout: string) {
        config['callout'] = callout;
        write_config();
    }

    async function read_placeholder() {
        let web_apps = config['placeholder'];
        if (web_apps == undefined) {
            config['placeholder'] = "Hi，Ombra！";
            write_config();
        }
        return config['placeholder'];
    }

    async function write_placeholder(placeholder: string) {
        config['placeholder'] = placeholder;
        write_config();
    }

    async function read_appinfo() {
        let web_apps = config['appinfo'];
        if (web_apps == undefined) {
            config['appinfo'] = [];
            write_config();
        }
        return config['appinfo'] as Array<CfgAppInfo>;
    }

    async function write_appinfo(appinfo: Array<CfgAppInfo>) {
        config['appinfo'] = appinfo;
        write_config();
    }
    return { init, read_web_apps, write_web_apps, read_appinfo, write_appinfo, read_local_app, write_local_app, read_callout, write_callout, read_placeholder, write_placeholder };
})