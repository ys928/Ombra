import { add_app } from "./global";
import File from '~/api/file'
import Window from "./api/window";
import Dialog from "./api/dialog";
import Ombra from "./api/ombra";
import Url from "./api/url";
import CLI from "./api/cli";
import App from "./api/app";
const list = [
    {
        name: '检测系统更新',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'ms-settings:windowsupdate']);
        }
    }, {
        name: 'WLAN',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'ms-settings:network-wifi']);
        }
    }, {
        name: 'VPN',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'ms-settings:network-vpn']);
        }
    }, {
        name: '日期与时间',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'ms-settings:dateandtime']);
        }
    }, {
        name: '语言与区域',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'ms-settings:regionlanguage']);
        }
    }, {
        name: '通知',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'ms-settings:notifications']);
        }
    }
    , {
        name: '睡眠',
        icon: '/imgs/sleep.png',
        feature: [],
        setup: () => {
            CLI.exec(['shutdown', '-h']);
        }
    }, {
        name: '关机',
        icon: '/imgs/shutdown.png',
        feature: [],
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
        setup: () => {
            CLI.exec(['explorer']);
        }
    }, {
        name: '锁屏',
        icon: '/imgs/lock.png',
        feature: [],
        setup: () => {
            CLI.exec(['rundll32.exe', 'user32.dll,LockWorkStation']);
        }
    }, {
        name: '回收站',
        icon: '/imgs/recyclebin.png',
        feature: [],
        setup: () => {
            CLI.exec(['start', 'shell:RecycleBinFolder']);
        }
    }, {
        name: 'CMD',
        icon: '/imgs/cmd.png',
        feature: ['explorer'],
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
        setup: () => {
            CLI.exec(['start', 'SystemPropertiesAdvanced'])
        }
    },
    {
        name: '编辑用户环境变量',
        icon: '/imgs/rundll32.png',
        feature: [''],
        setup: () => {
            CLI.exec(['start', 'rundll32', 'sysdm.cpl,EditEnvironmentVariables'])
        }
    }
];

export async function load_sys_app() {
    //仅限主窗口执行加载app代码
    if (Window.label != "MainWindow") {
        return;
    }
    //加载系统应用功能
    for (let app of list) {
        add_app(app.name, '', app.icon, app.feature, false, app.setup, false, '');
    }
    //加载用户安装的应用
    let apps = await App.get_all();
    for (let i = 0; i < apps.length; i++) {
        let feature: string[] = [];
        if (apps[i].name == 'Visual Studio Code') {
            feature.push('explorer');
        }
        let exist = await File.exists(apps[i].icon);
        let icon_url = exist ? Url.convert(apps[i].icon) : "/logo.png";
        add_app(apps[i].name, '', icon_url, feature, false, () => {
            let features = Ombra.get_features();
            let text = Ombra.get_text();
            if (features.includes('explorer')) {
                CLI.exec(['start', apps[i].start, text])
            } else {
                CLI.exec(['start', apps[i].start])
            }
        }, false, '');

    }
};