import { dialog, invoke, path } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { os } from '@tauri-apps/api'
import { OpenDialogOptions } from "@tauri-apps/api/dialog";

/**
 * 说明：本文件为ombra的全局接口文件，包含了目前所有可用的api。
 * 为了方便使用，api的命名规范采用 名词_动词 的形式，例如打开文件的函数为：file_open
 * 目前包含的名词有：
 * om：指代ombra，用于向插件传递数据
 * dir：目录
 * app：本程序中的app
 * cli：命令行接口（command line interface）
 * url：网址链接
 * dlg：对话框（dialog）
 * exp：资源管理器（explorer）
 */

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

