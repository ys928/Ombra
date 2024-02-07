import { invoke } from "@tauri-apps/api";

namespace Ombra {
    /**
     * 
     * @returns 获取输入插件的文本内容（大部分情况下都是用户输入搜索框中的内容）
     */
    export function get_text() {
        let text = localStorage.getItem('search_text');
        if (text == null) return '';
        return text;
    }

    /**
     * 
     * @returns 获取此时的feature列表，可被用于判断执行何种操作
     */
    export function get_features() {
        let text = localStorage.getItem('search_features');
        if (text == null) return [] as Array<string>;
        return JSON.parse(text) as Array<string>;
    }

    /**
     * 
     * @returns 设置输入插件的内容（仅限ombra内部使用，插件使用无效）
     */
    export function set_text(text: string) {
        localStorage.setItem('search_text', text);
    }

    /**
     * 
     * @returns 设置此时的feature列表（仅限ombra内部使用，插件使用无效）
     */
    export function set_features(features: Array<string>) {
        localStorage.setItem('search_features', JSON.stringify(features));
    }

    /**
     * 
     * @param appid 设置当前将要打开的app id（仅限ombra内部使用，插件使用无效）
     */
    export function set_appid(appid: string) {
        localStorage.setItem('appid', appid);
    }

    /**
     * 
     * @param appid 设置当前将要打开的app id（仅限ombra内部使用，插件使用无效）
     */
    export function get_appid() {
        let appid = localStorage.getItem('appid');
        if (appid == null) return '';
        return appid;
    }
    /**
     * 
     * @param index 设置当前要打开的插件代码文件index.html位置
     */
    export function set_plugin_index(index: string) {
        localStorage.setItem('plugin_index', index);
    }

    /**
     * 
     * @returns 获取当前要打开的插件代码文件index.html位置
     */
    export function get_plugin_index() {
        let appid = localStorage.getItem('plugin_index');
        if (appid == null) return '';
        return appid;
    }

    /**
     * 
     * @param hans 汉字，如“世界”
     * @returns 返回汉字对应的拼音数组，如['shi','jie']
     */
    export function to_pinyin(hans: string) {
        return invoke<Array<string>>('to_pinyin', { hans: hans });
    }
}

export default Ombra;