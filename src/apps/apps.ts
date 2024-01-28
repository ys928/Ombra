import AppFileSearch from "./FileSearch/App";
import AppOpenLink from "./OpenLink/App"
import AppOpenPath from "./OpenPath/App"
import AppOpenFile from "./OpenFile/App"
import AppPluginDevelopTool from './PluginDevelopTool/App'
import { path } from "@tauri-apps/api";
import File from '~/api/file'
import Directory from "../api/dir";
import Url from "../api/url";
import Dialog from "../api/dialog";
import Ombra from "../api/ombra";
import CLI from "../api/cli";
import App from "../api/app";
import { useAppListStore } from '~/stores/appList';

let user_apps_list = [
    AppFileSearch,
    AppOpenLink,
    AppOpenPath,
    AppOpenFile,
    AppPluginDevelopTool
]

export async function load_apps() {
    const applistStore = useAppListStore();

    //加载内嵌应用
    for (let app of user_apps_list) {
        app.preload();
        applistStore.add(app.name, app.id, app.icon, app.feature, app.component, app.setup, app.only_feature);
    }

    //加载插件应用

    let plugin_path = await path.resolve('./plugin');
    let plugin_dirs = await Directory.walk(plugin_path);
    for (let i = 0; i < plugin_dirs.length; i++) {
        let name = '';
        let icon = '';
        let plugin_index = '';
        let id = ''
        let features = [];
        let plugin_files = await Directory.walk(plugin_dirs[i].path + '\\' + plugin_dirs[i].name);
        for (let f of plugin_files) {
            if (f.name.startsWith('icon')) { //icon图标
                icon = Url.convert(f.path + '\\' + f.name);
            } else if (f.name == 'index.html') {
                plugin_index = Url.convert(f.path + '\\' + f.name);
            } else if (f.name == 'config.json') {
                let text = await File.read_text(f.path + '\\' + f.name);
                let config = JSON.parse(text);
                features = config.features;
                id = config.id;
                name = config.name;
            }
        }
        applistStore.add(name, id, icon, features, plugin_index, () => { }, false);
    }

    //加载系统应用、功能

    for (let app of sys_seting_list) {
        applistStore.add(app.name, '', app.icon, app.feature, null, app.setup, false);
    }
    //加载用户安装的应用
    let apps = await App.get_sys();
    for (let i = 0; i < apps.length; i++) {
        let feature: string[] = [];
        if (apps[i].name == 'Visual Studio Code') {
            feature.push('explorer');
        }
        let exist = await File.exists(apps[i].icon);
        let icon_url = exist ? Url.convert(apps[i].icon) : "/logo.png";
        applistStore.add(apps[i].name, '', icon_url, feature, null, () => {
            let features = Ombra.get_features();
            let text = Ombra.get_text();
            if (features.includes('explorer')) {
                if (apps[i].start.startsWith('shell')) {
                    CLI.exec(['start', apps[i].start, text])
                } else {
                    CLI.exec(['start', '', apps[i].start, text])
                }
            } else {
                if (apps[i].start.startsWith('shell')) {
                    CLI.exec(['start', apps[i].start])
                } else {
                    CLI.exec(['start', '', apps[i].start])
                }
            }
        }, false);

    }
    //加载web应用
    let web_apps = await App.get_web();
    for (let w of web_apps) {
        if (!w.on) continue; //跳过未启用的web app
        applistStore.add_web(w);
    }
}

const sys_seting_list = [
    {
        name: '检测系统更新',
        icon: '/imgs/windows-setting.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'ms-settings:windowsupdate']);
        }
    }, {
        name: 'WLAN',
        icon: '/imgs/windows-setting.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'ms-settings:network-wifi']);
        }
    }, {
        name: 'VPN',
        icon: '/imgs/windows-setting.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'ms-settings:network-vpn']);
        }
    }, {
        name: '日期与时间',
        icon: '/imgs/windows-setting.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'ms-settings:dateandtime']);
        }
    }, {
        name: '语言与区域',
        icon: '/imgs/windows-setting.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'ms-settings:regionlanguage']);
        }
    }, {
        name: '通知',
        icon: '/imgs/windows-setting.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'ms-settings:notifications']);
        }
    }
    , {
        name: '睡眠',
        icon: '/imgs/sleep.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['shutdown', '-h']);
        }
    }, {
        name: '关机',
        icon: '/imgs/shutdown.png',
        feature: [],
        self: false,
        setup: async () => {
            let ret = await Dialog.confirm('确定要执行【关机】？', "Ombra", 'warning');
            if (ret) {
                CLI.exec(['shutdown', '-s', '-t', '0']);
            }
        }
    }, {
        name: '重启',
        icon: '/imgs/reboot.png',
        feature: [],
        self: false,
        setup: async () => {
            let ret = await Dialog.confirm('确定要执行【重启】？', "Ombra", 'warning');
            if (ret) {
                CLI.exec(['shutdown', '-r']);
            }
        }
    }, {
        name: '注销',
        icon: '/imgs/logout.png',
        feature: [],
        self: false,
        setup: async () => {
            let ret = await Dialog.confirm('确定执行【注销】，退出当前账户登录？', "Ombra", 'warning');
            if (ret) {
                CLI.exec(['shutdown', '-l']);
            }
        }
    },
    {
        name: '文件资源管理器',
        icon: '/imgs/explorer.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['explorer']);
        }
    }, {
        name: '锁屏',
        icon: '/imgs/lock.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['rundll32.exe', 'user32.dll,LockWorkStation']);
        }
    }, {
        name: '回收站',
        icon: '/imgs/recyclebin.png',
        feature: [],
        self: false,
        setup: () => {
            CLI.exec(['start', 'shell:RecycleBinFolder']);
        }
    }, {
        name: 'CMD',
        icon: '/imgs/cmd.png',
        feature: ['explorer'],
        self: false,
        setup: () => {
            let features = Ombra.get_features();
            let text = Ombra.get_text();
            if (features.includes('explorer')) {
                CLI.exec(['start', 'cmd', '/k', 'cd', '/d', text])
            } else {
                CLI.exec(['start', 'cmd'])
            }
        }
    }, {
        name: 'PowerShell',
        icon: '/imgs/powershell.png',
        feature: ['explorer'],
        self: false,
        setup: () => {
            let features = Ombra.get_features();
            let text = Ombra.get_text();
            if (features.includes('explorer')) {
                CLI.exec(['powershell', '-Command', 'Set-Location', '-Path ', text, ';Start-Process', 'PowerShell'])
            } else {
                CLI.exec(['powershell', '-Command', 'Start-Process', 'PowerShell'])
            }
        }
    },
    {
        name: '系统属性环境变量',
        icon: '/imgs/rundll32.png',
        feature: [''],
        self: false,
        setup: () => {
            CLI.exec(['start', 'SystemPropertiesAdvanced'])
        }
    },
    {
        name: '编辑用户环境变量',
        icon: '/imgs/rundll32.png',
        feature: [''],
        self: false,
        setup: () => {
            CLI.exec(['start', 'rundll32', 'sysdm.cpl,EditEnvironmentVariables'])
        }
    }
];