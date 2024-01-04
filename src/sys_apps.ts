import { add_app } from "./global";
import { app_get, app_start, cli_exec, dlg_confirm, file_convert, om_get_features, om_get_text, win_label } from "./ombra";

const list = [
    {
        name: '设置',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            cli_exec(['start', 'ms-settings:']);
        }
    }, {
        name: '检测系统更新',
        icon: '/imgs/windows-setting.png',
        feature: [],
        setup: () => {
            cli_exec(['start', 'ms-settings:windowsupdate']);
        }
    }
    , {
        name: '睡眠',
        icon: '/imgs/sleep.png',
        feature: [],
        setup: () => {
            cli_exec(['shutdown', '-h']);
        }
    }, {
        name: '关机',
        icon: '/imgs/shutdown.png',
        feature: [],
        setup: async () => {
            let ret = await dlg_confirm('确定要执行【关机】？', "Ombra", 'warning');
            if (ret) {
                cli_exec(['shutdown', '-s', '-t', '0']);
            }
        }
    }, {
        name: '重启',
        icon: '/imgs/reboot.png',
        feature: [],
        setup: async () => {
            let ret = await dlg_confirm('确定要执行【重启】？', "Ombra", 'warning');
            if (ret) {
                cli_exec(['shutdown', '-r']);
            }
        }
    }, {
        name: '注销',
        icon: '/imgs/logout.png',
        feature: [],
        setup: async () => {
            let ret = await dlg_confirm('确定执行【注销】，退出当前账户登录？', "Ombra", 'warning');
            if (ret) {
                cli_exec(['shutdown', '-l']);
            }
        }
    },
    {
        name: '文件资源管理器',
        icon: '/imgs/explorer.png',
        feature: [],
        setup: () => {
            cli_exec(['explorer']);
        }
    }, {
        name: '锁屏',
        icon: '/imgs/lock.png',
        feature: [],
        setup: () => {
            cli_exec(['rundll32.exe', 'user32.dll,LockWorkStation']);
        }
    }, {
        name: '回收站',
        icon: '/imgs/recyclebin.png',
        feature: [],
        setup: () => {
            cli_exec(['start', 'shell:RecycleBinFolder']);
        }
    }, {
        name: 'CMD',
        icon: '/imgs/cmd.png',
        feature: ['explorer'],
        setup: (text: string, features: string[]) => {
            if (features.includes('explorer')) {
                cli_exec(['start', 'cmd', '/k', 'cd', '/d', text])
            } else {
                cli_exec(['start', 'cmd'])
            }
        }
    }, {
        name: 'PowerShell',
        icon: '/imgs/powershell.png',
        feature: ['explorer'],
        setup: (text: string, features: string[]) => {
            if (features.includes('explorer')) {
                cli_exec(['powershell', '-Command', 'Set-Location', '-Path ', text, ';Start-Process', 'PowerShell'])
            } else {
                cli_exec(['powershell', '-Command', 'Start-Process', 'PowerShell'])
            }
        }
    }
];

export async function load_sys_app() {
    //仅限主窗口执行加载app代码
    if (win_label != "MainWindow") {
        return;
    }
    //加载系统应用功能
    for (let app of list) {
        add_app(app.name, '', app.icon, app.feature, false, app.setup, false, '');
    }
    //加载用户安装的应用
    let apps = await app_get();
    for (let i = 0; i < apps.length; i++) {
        let feature: string[] = [];
        if (apps[i].name == 'Visual Studio Code') {
            feature.push('explorer');
        }
        add_app(apps[i].name, '', file_convert(apps[i].icon), feature, false, () => {
            let features = om_get_features();
            let text = om_get_text();
            if (features.includes('explorer')) {
                cli_exec([apps[i].path, text])
            } else {
                app_start(apps[i].path);
            }
        }, false, '');

    }
};