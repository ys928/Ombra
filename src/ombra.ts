import { clipboard, dialog, fs, globalShortcut, invoke, path } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { LogicalSize, WebviewWindow, appWindow } from "@tauri-apps/api/window";
import { os } from '@tauri-apps/api'
import { OpenDialogOptions } from "@tauri-apps/api/dialog";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

/**
 * 说明：本文件为ombra的全局接口文件，包含了目前所有可用的api。
 * 为了方便使用，api的命名规范采用 名词_动词 的形式，例如打开文件的函数为：file_open
 * 目前包含的名词有：
 * om：指代ombra，用于向插件传递数据
 * file：文件
 * path：路径
 * dir：目录
 * app：本程序中的app
 * cli：命令行接口（command line interface）
 * url：网址链接
 * dlg：对话框（dialog）
 * win：当前窗口（window）
 * exp：资源管理器（explorer）
 * clip：剪切板（clipboard）
 * gs：全局快捷键（Global Shortcut）
 * notif：系统提示（notification）
 */

/**
 * @description 判断路径是文件还是目录
 * @param path 要判断文件的路径
 * @returns dir、file、or error
 */
export async function path_judge(path: string) {
    return await invoke<string>('dir_or_file', { path: path });
}

/**
 * 
 * @param path 路径
 * @returns 返回该路径是否存在
 */
export async function path_exist(path: string) {
    return fs.exists(path);
}

/**
 * 
 * @param paths 多个路径
 * @returns 返回拼接好的路径
 */
export function path_join(...paths: string[]) {
    return path.join(...paths);
}

/**
 * @description 启动可执行文件
 * @param path 可执行文件路径
 */
export async function app_start(path: string) {
    if (await os.platform() == 'win32') {
        cli_exec(['start', '', path]);
    }
}
/**
 * @description 用命令行执行一个命令，
 * @param command 要执行的命令，用数组的形式组织命令与各参数，比如 ls -l 的形式应该为：['ls','-l']
 */
export function cli_exec(command: Array<string>) {
    invoke('cmd_exec', { args: command });
}
/**
 * @description 用系统默认浏览器打开链接
 * @param url 网络链接
 */
export function url_open(url: string) {
    invoke('open_web_url', { url: url });
}

type AppInfo = {
    name: string,
    icon: string,
    start: string,
}
/**
 * @description 获取当前系统所有已安装的app
 * @returns 返回app列表数据，每一项包含名称：name、路径：path、图标位置：icon
 */
export async function app_get() {
    const get_apps_result = new Promise<Array<AppInfo>>((resolve) => {
        //获取所有遍历到的程序
        listen<Array<AppInfo>>('get_all_app_result', (e) => {
            resolve(e.payload)
        });
    });
    invoke('get_all_app');
    return await get_apps_result;
}

/**
 * 
 * @returns 判断当前app是否处于嵌入模式
 */
export function app_is_embed() {
    return appWindow.label == 'MainWindow';
}


/**
 * @description 弹出一个确认对话框窗口
 * @param msg 窗口内容
 * @param title 窗口标题
 * @param type 类型
 * @param okLabel 确认按钮文本
 * @param cancelLabel 取消按钮文本
 * @returns true代表按下确认，否则代表按下取消
 */
export async function dlg_confirm(msg: string, title: string, type: "warning" | "info" | "error", okLabel = '确定', cancelLabel = '取消') {
    return dialog.confirm(msg, { title: title, type: type, okLabel: okLabel, cancelLabel: cancelLabel })
}
/**
 * @description 弹出对话框选择文件打开
 * @param option 一些选项
 * @returns 返回选择结果路径，为null则说明没有选择
 */
export async function dlg_open(option: OpenDialogOptions) {
    return dialog.open(option);
}

/**
 * @description 以utf-8编码读取文本所有内容并返回
 * @param filepath 文件路径
 * @returns 文本内容
 */
export async function file_read_text(filepath: string) {
    return fs.readTextFile(filepath);
}

/**
 * 
 * @param filepath 文件路径
 * @param content 要写入文本的内容
 */
export function file_write_text(filepath: string, content: string) {
    fs.writeTextFile(filepath, content);
}
/**
 * 
 * @param path 文件路径
 * @returns 文件是否存在
 */
export function file_exists(path: string) {
    return fs.exists(path);
}

/**
 * @description tauri不支持直接在窗口加载文件，比如图片，若要在窗口中加载图片之类的本地文件，需要使用该函数进行路径转换
 * @param filepath 电脑文件路径
 * @returns 返回可被窗口加载的路径
 */
export function file_convert(filepath: string) {
    return convertFileSrc(filepath);
}

/**
 * 
 * @returns 返回当前电脑的配置目录
 */
export function dir_config() {
    return path.appConfigDir();
}

type FileInfo = {
    name: string,
    path: string,
    time: number,
    ftype: number, //1：文件、2：目录
}

/**
 * 
 * @param path 要遍历的目录
 * @param level 要遍历的层级，默认为1层，为0则递归遍历目录下的所有文件
 * @returns 返回文件信息列表
 */
export async function dir_walk(path: string, level = 1) {
    let result = new Promise<Array<FileInfo>>((resolve) => {
        listen<Array<FileInfo>>('walk_dir_result', (e) => {
            resolve(e.payload);
        })
    });
    invoke('walk_dir', {
        path: path,
        level: level
    });
    return result;
}

/**
 * 当前窗口的label
 */
export const win_label = appWindow.label;
/**
 * 关闭当前窗口
 */
export function win_close() {
    if (appWindow.label == 'MainWindow') {
        win_set_size(170);
        window.location.href = "/";
    } else {
        appWindow.close();
    }
}
/**
 * @description 显示当前窗口
 */
export function win_show() {
    appWindow.show();
}

/**
 * 最小化当前窗口
 */
export function win_min() {
    appWindow.minimize();
}

/**
 * 切换到最大化或恢复正常
 */
export function win_toggle_max() {
    appWindow.toggleMaximize();
}
/**
 * 从插件面板退出、切换回主搜索面板（仅限嵌入模式下可用）
 */
export function win_to_main() {
    if (appWindow.label == 'MainWindow') {
        win_set_size(170);
        win_set_resizable(false);
        window.location.href = "/";
    }
}
/**
 * 
 */
/**
 * @description 从主搜索面板切换到app面板（大部分情况下应仅限ombra内部使用，且仅嵌入模式下可用）
 * @param appid app的id
 */
export function win_to_app(appid: string) {
    if (appWindow.label == 'MainWindow') {
        win_set_size(600);
        win_set_resizable(true);
        window.location.href = '/apps/' + appid;
    }
}
/**
 * @description 从主搜索面板窗口中新开一个app窗口（大部分情况下应仅限ombra内部使用）
 * @param appid app的id
 */
export function win_new_app(appid: string) {
    if (appWindow.label == 'MainWindow') {
        appWindow.hide();
        window.location.href = '/';
        let w = new WebviewWindow(appid, {
            url: '/apps/' + appid,
            decorations: false,
            transparent: true
        });
        w.setSize(new LogicalSize(800, 600));
        w.setResizable(true);
    }
}
/**
 * 窗口阴影、圆滑边框模式，仅限非主窗口使用
 */
export function win_shadow() {
    if (appWindow.label != 'MainWindow') {
        invoke('shadow_window');
    }
}

/**
 * @description 隐藏当前窗口
 */
export function win_hide() {
    appWindow.hide();
}
/**
 * @description 聚焦当前窗口
 */
export function win_focus() {
    appWindow.setFocus();
}
/**
 * 
 * @returns 当前窗口是否可见
 */
export async function win_is_visible() {
    return appWindow.isVisible();
}

/**
 * 
 * @returns 判断当前窗口是否是主搜索窗口
 */
export function win_is_main() {
    return appWindow.label == 'MainWindow';
}

/**
 * @description 设置当前窗口的大小
 * @param h 窗口的高度，默认主窗口高度170，app窗口推荐设置为600
 * @param w 窗口的宽度，默认800
 */
export function win_set_size(h = 170, w = 800) {
    appWindow.setSize({
        type: 'Logical',
        width: w,
        height: h,
    });
}
/**
 * 
 * @param resizable 当前窗口是否可调整大小
 */
export function win_set_resizable(resizable: boolean) {
    appWindow.setResizable(resizable);
}

/**
 * 
 * @param label 窗口label
 * @param callback 当该窗口失去聚焦时执行函数，
 */
export function win_event_blur(label: string, callback: Function) {
    return listen('tauri://blur', (e) => {
        if (e.windowLabel == label) {
            callback();
        }
    });
}

/**
 * 
 * @param label 窗口label
 * @param callback 当该窗口聚焦时执行函数，
 */
export function win_event_focus(label: string, callback: Function) {
    return listen('tauri://focus', (e) => {
        if (e.windowLabel == label) {
            callback();
        }
    });
}

/**
 * @description 用电脑默认应用打开该文件
 * @param path 文件路径
 */
export function exp_open_file(path: string) {
    invoke('default_open_file', { path: path });
}
/**
 * @description 打开资源管理器选择到指定文件
 * @param path 文件路径
 */
export function exp_select_file(path: string) {
    invoke('explorer_select_path', { path: path });
}

/**
 * 
 * @returns 返回当前聚焦的explorer窗口显示的路径
 */
export async function exp_get_path() {
    return await invoke<string>('get_explorer_show_path')
}

/**
 * 
 * @returns 返回剪切板中的文本
 */
export async function clip_get_text() {
    return clipboard.readText();
}
/**
 * 
 * @param text 设置剪切板的文本
 */
export async function clip_set_text(text: string) {
    clipboard.writeText(text);
}

/**
 * 
 * @param shortkey 要注册的全局快捷键
 * @param callback 相应的回调函数
 */
export function gs_register(shortkey: string, callback: Function) {
    globalShortcut.register(shortkey, () => {
        callback();
    })
}
/**
 * 
 * @param shortkey 取消注册的全局快捷键
 */
export function gs_unregister(shortkey: string) {
    globalShortcut.unregister(shortkey);
}
/**
 * 
 * @param shortkey 全局快捷键
 * @returns 是否已经被注册
 */
export async function gs_is_registered(shortkey: string) {
    return globalShortcut.isRegistered(shortkey);
}

/**
 * 
 * @returns 获取输入插件的文本内容（大部分情况下都是用户输入搜索框中的内容）
 */
export function om_get_text() {
    let text = localStorage.getItem('search_text');
    if (text == null) return '';
    return text;
}

/**
 * 
 * @returns 获取此时的feature列表，可被用于判断执行何种操作
 */
export function om_get_features() {
    let text = localStorage.getItem('search_features');
    if (text == null) return [] as Array<string>;
    return JSON.parse(text) as Array<string>;
}

/**
 * 
 * @returns 设置输入插件的内容（仅限ombra内部使用，插件使用无效）
 */
export function om_set_text(text: string) {
    localStorage.setItem('search_text', text);
}

/**
 * 
 * @returns 设置此时的feature列表（仅限ombra内部使用，插件使用无效）
 */
export function om_set_features(features: Array<string>) {
    localStorage.setItem('search_features', JSON.stringify(features));
}

/**
 * 
 * @param appid 设置当前将要打开的app id（仅限ombra内部使用，插件使用无效）
 */
export function om_set_appid(appid: string) {
    localStorage.setItem('appid', appid);
}

/**
 * 
 * @param appid 设置当前将要打开的app id（仅限ombra内部使用，插件使用无效）
 */
export function om_get_appid() {
    let appid = localStorage.getItem('appid');
    if (appid == null) return '';
    return appid;
}
/**
 * 
 * @param index 设置当前要打开的插件代码文件index.html位置
 */
export function om_set_plugin_index(index: string) {
    localStorage.setItem('plugin_index', index);
}

/**
 * 
 * @returns 获取当前要打开的插件代码文件index.html位置
 */
export function om_get_plugin_index() {
    let appid = localStorage.getItem('plugin_index');
    if (appid == null) return '';
    return appid;
}

/**
 * 
 * @param hans 汉字，如“世界”
 * @returns 返回汉字对应的拼音数组，如['shi','jie']
 */
export async function om_to_pinyin(hans: string) {
    return await invoke<Array<string>>('to_pinyin', { hans: hans });
}

/**
 * 
 * @returns 当前是否允许发送提示消息
 */
export async function notif_is_grant() {
    return await isPermissionGranted();
}
/**
 * 
 * @returns 尝试请求获取发送系统提示的权限
 */
export async function notif_request() {
    const permission = await requestPermission();
    return permission === 'granted';
}

/**
 * @description 发送系统提示消息
 * @param title 标题
 * @param content 内容
 */
export function notif_send(title: string, content: string) {
    sendNotification({ title: title, body: content });
}
