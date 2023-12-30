import { reactive } from "vue";
import { dir_config, file_read_text, file_write_text, gs_is_registered, gs_register, gs_unregister, path_exist, win_hide, win_set_resizable, win_set_size } from "./ombra";

export type AppInfo = {
    name: string, //应用名，将显示在应用菜单面板上
    id: string, //应用id，如果self属性为true，那么该选项必填，否则可选，将被用于独立窗口label、以及vue中的路由
    self: boolean, //是否需要独立窗口功能
    icon: string,   //图标所在路径
    feature: Array<string>, //该应用可处理的特点，当用户输出指定特点的内容时将被显示在推荐页，目前支持：filename、url、dir_path、file_path,explorer
    only_feature: boolean, //是否只在触发特点时才显示
    weight: number, //应用权重，随用户使用次数进行计算
    setup: () => void, //当用户点击该app时将被立即执行的函数，text为用户当前输入的文本,feature为当前用户输入匹配到的特性
}

const app_list = reactive([]) as Array<AppInfo>

export function get_app_list() {
    return app_list;
}

export function add_app(name: string, id: string, icon: string, feature: Array<string>, self: boolean, callback: Function, only_feature = false) {
    app_list.push({
        name: name,
        icon: icon,
        feature: feature,
        only_feature: only_feature,
        weight: 0,
        id: id,
        self: self,
        setup: () => {
            if (self) {
                win_set_size(600);
                win_set_resizable(true);
            } else {
                win_hide();
            }
            callback();
        }
    });
}

export function time_ago(fromTimestamp: number) {
    const currentTimestamp = Math.floor(Date.now() / 1000);
    const secondsAgo = currentTimestamp - fromTimestamp;

    // 定义时间单位
    const minute = 60;
    const hour = minute * 60;
    const day = hour * 24;
    const week = day * 7;
    const month = day * 30;
    const year = day * 365;

    // 根据时间距离返回不同的字符串
    if (secondsAgo < minute) {
        return `${secondsAgo}秒前`;
    } else if (secondsAgo < hour) {
        const minutesAgo = Math.floor(secondsAgo / minute);
        return `${minutesAgo}分钟前`;
    } else if (secondsAgo < day) {
        const hoursAgo = Math.floor(secondsAgo / hour);
        return `${hoursAgo}小时前`;
    } else if (secondsAgo < week) {
        const daysAgo = Math.floor(secondsAgo / day);
        return `${daysAgo}天前`;
    } else if (secondsAgo < month) {
        const weeksAgo = Math.floor(secondsAgo / week);
        return `${weeksAgo}周前`;
    } else if (secondsAgo < year) {
        const monthsAgo = Math.floor(secondsAgo / month);
        return `${monthsAgo}月前`;
    } else {
        const yearsAgo = Math.floor(secondsAgo / year);
        return `${yearsAgo}年前`;
    }
}


//将以秒为单位的时间戳转化为字符串形式
export function time_to_str(timestamp: number) {
    const milliseconds = timestamp * 1000;
    const date = new Date(milliseconds);
    const options = {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: 'numeric',
        minute: 'numeric',
        hour12: false // 24小时制
    };
    //@ts-ignore
    return date.toLocaleString(undefined, options).replace(',', '');
}

async function get_config_path() {
    let p = await dir_config();
    p += 'config.json';
    if (!await path_exist(p)) {
        file_write_text(p, '{}');
    }
    return p;
}

async function read_config() {
    let p = await get_config_path();
    let config = await file_read_text(p);
    let js_config = JSON.parse(config);
    return js_config;
}

async function write_config(config: any) {
    let p = await get_config_path();
    let str = JSON.stringify(config);
    file_write_text(p, str);
}

export async function read_config_item(item: string) {
    let cfg = await read_config();
    return cfg[item];
}

export async function write_config_item(item: string, cnt: any) {
    let cfg = await read_config();
    cfg[item] = cnt;
    write_config(cfg);
}

//设置面板呼出快捷键
export async function set_shortcut(shortkey: string, func: Function) {
    const isreg = await gs_is_registered(shortkey);
    if (isreg) {
        await gs_unregister(shortkey);
    }
    //注册全局快捷键
    gs_register(shortkey, () => {
        func();
    })
}

export function get_span(cnt: string, cla: string) {
    return `<span class="${cla}">${cnt}</span>`;
}

//全局计时任务

let timing_task = [] as Array<{
    setup: Function,
    time: number, //多少分钟执行一次
    pass: number //已经过去多少分钟
}>;

setInterval(() => {
    for (let i = 0; i < timing_task.length; i++) {
        if (timing_task[i].pass < timing_task[i].time) {
            timing_task[i].pass++;
        } else {
            timing_task[i].pass = 0;
            timing_task[i].setup();
        }
    }
}, 60 * 1000); //每分钟枚举一次计时任务表

export function add_timing_task(time: number, setup: Function) {
    timing_task.push({
        setup: setup,
        time: time,
        pass: 0
    });
}