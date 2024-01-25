import Directory from "./directory";
import Path from "./path";
import File from "./file";
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

export default class Config {
    static async path() {
        let p = await Directory.config();
        p += 'config.json';
        if (!await Path.exists(p)) {
            File.write_text(p, '{}');
        }
        return p;
    }
    static async read_item(item: string) {
        let cfg = await read_config();
        return cfg[item];
    }

    static async write_item(item: string, cnt: any) {
        let cfg = await read_config();;
        cfg[item] = cnt;
        write_config(cfg);
    }

}