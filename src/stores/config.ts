import { defineStore } from "pinia";
import { Dir, Path, FS } from "~/api";

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
    callout: 'CommandOrControl+Space',
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

    async function write_config() {
        let p = await Dir.config();
        p = await Path.join(p, 'config.json');
        FS.write_text(p, JSON.stringify(config));
    }
    async function read_config() {
        let p = await Dir.config();
        p = await Path.join(p, 'config.json');
        const text = await FS.read_text(p);
        // console.log(text);
        if (text.length == 0) {
            return {} as Config;
        } else {
            return JSON.parse(text) as Config;
        }
    }

    async function read_web_apps() {
        // console.log("read_web_apps");
        if (config.web_apps == undefined) {
            const cfg = await read_config();
            if (cfg.web_apps == undefined) {
                config.web_apps = default_config.web_apps;
                write_config();
            } else {
                config.web_apps = cfg.web_apps;
            }
        }
        return config.web_apps;
    }

    async function write_web_apps(cnt: Array<WebUrlApp>) {
        // console.log('write_web_apps')
        config.web_apps = cnt;
        write_config();
    }

    async function read_local_app() {
        // console.log('read_local_app');
        if (config.local_apps == undefined) {
            const cfg = await read_config();
            if (cfg.local_apps == undefined) {
                config.local_apps = default_config.local_apps;
                write_config();
            } else {
                config.local_apps = cfg.local_apps;
            }
        }
        return config.local_apps;
    }

    async function write_local_app(cnt: Array<LocalApp>) {
        // console.log('Writing local app');
        config.local_apps = cnt;
        write_config();
    }

    async function read_callout() {
        // console.log("Reading callout");
        if (config.callout == undefined) {
            const cfg = await read_config();
            if (cfg.callout == undefined) {
                config.callout = default_config.callout;
                write_config();
            } else {
                config.callout = cfg.callout;
            }
        }
        return config.callout;
    }
    async function write_callout(callout: string) {
        // console.log('write_callout');
        config.callout = callout;
        write_config();
    }

    async function read_placeholder() {
        // console.log('read_placeholder');
        if (config.placeholder == undefined) {
            const cfg = await read_config();
            if (cfg.placeholder == undefined) {
                config.placeholder = default_config.placeholder;
                write_config();
            } else {
                config.placeholder = cfg.placeholder;
            }
        }
        return config.placeholder;
    }

    async function write_placeholder(placeholder: string) {
        // console.log('writing placeholder');
        config.placeholder = placeholder;
        write_config();
    }

    async function read_appinfo() {
        // console.log('reading appinfo');
        if (config.appinfo == undefined) {
            const cfg = await read_config();
            if (cfg.appinfo == undefined) {
                config.appinfo = default_config.appinfo;
                write_config();
            } else {
                config.appinfo = cfg.appinfo;
            }
        }
        return config.appinfo;
    }

    async function write_appinfo(appinfo: Array<CfgAppInfo>) {
        // console.log('write_appinfo');
        config.appinfo = appinfo;
        write_config();
    }
    return { read_web_apps, write_web_apps, read_appinfo, write_appinfo, read_local_app, write_local_app, read_callout, write_callout, read_placeholder, write_placeholder };
})