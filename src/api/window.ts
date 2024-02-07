import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { LogicalSize, WebviewWindow, appWindow } from "@tauri-apps/api/window";

namespace Window {
    /**
     * 当前窗口的label
     */
    export const label = appWindow.label;
    /**
     * 关闭当前窗口
     */
    export function close() {
        if (is_main()) {
            set_height(170);
            window.router.push('/');
        } else {
            appWindow.close();
        }
    }
    /**
     * @description 显示当前窗口
     */
    export function show() {
        appWindow.show();
    }

    /**
     * 最小化当前窗口
     */
    export function min() {
        appWindow.minimize();
    }

    /**
     * 切换到最大化或恢复正常
     */
    export function toggle_max() {
        appWindow.toggleMaximize();
    }
    /**
     * @description 从主搜索面板窗口中新开一个app窗口（大部分情况下应仅限ombra内部使用）
     * @param appid app的id
     */
    export function new_app(appid: string) {
        if (!is_main()) return;
        hide();
        window.router.push('/');
        let w = new WebviewWindow(appid, {
            url: `/app?id=${appid}`,
            decorations: false,
            transparent: true
        });
        w.setSize(new LogicalSize(800, 600));
        w.setResizable(true);
    }
    /**
     * @description 将主窗口切换到app界面
     * @param appid app的id
     */
    export function to_app(appid: string) {
        if (!is_main()) return;
        Window.set_height(600);
        Window.set_resizable(true);
        window.router.push(`/app?id=${appid}`);
    }
    /**
     * @description 将主窗口切换到主搜索界面
     */
    export function to_main() {
        if (!is_main()) return;
        Window.set_height(170);
        Window.set_resizable(false);
        window.router.push(`/`);
    }
    /**
     * 
     * @returns 将主窗口切换到设置界面
     */
    export function to_setting() {
        if (!is_main()) return;
        Window.set_height(600);
        Window.set_resizable(false);
        window.router.push(`/setting`);
    }

    /**
     * 窗口阴影、圆滑边框模式，仅限非主窗口使用
     */
    export function shadow() {
        if (appWindow.label != 'MainWindow') {
            invoke('shadow_window');
        }
    }

    /**
     * @description 隐藏当前窗口
     */
    export function hide() {
        appWindow.hide();
    }
    /**
     * @description 聚焦当前窗口
     */
    export function focus() {
        appWindow.setFocus();
    }
    /**
     * 
     * @returns 当前窗口是否可见
     */
    export function is_visible() {
        return appWindow.isVisible();
    }

    /**
     * 
     * @returns 判断当前窗口是否是主搜索窗口
     */
    export function is_main() {
        return appWindow.label == 'MainWindow';
    }

    /**
     * @description 设置当前窗口的大小
     * @param w 窗口的宽度，默认800
     * @param h 窗口的高度，默认主窗口高度170，app窗口推荐设置为600
     */
    export function set_size(w = 800, h = 170) {
        appWindow.setSize({
            type: 'Logical',
            width: w,
            height: h,
        });
    }
    /**
     * @description 设置窗口高度，默认宽度800
     * @param h 窗口高度，推荐主窗口170，app窗口600
     */
    export function set_height(h = 170) {
        appWindow.setSize({
            type: 'Logical',
            width: 800,
            height: h,
        });
    }
    /**
     * 
     * @param resizable 当前窗口是否可调整大小
     */
    export function set_resizable(resizable: boolean) {
        appWindow.setResizable(resizable);
    }

    /**
     * 
     * @param label 窗口label
     * @param callback 当该窗口失去聚焦时执行函数，
     */
    export function event_blur(label: string, callback: Function) {
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
    export function event_focus(label: string, callback: Function) {
        return listen('tauri://focus', (e) => {
            if (e.windowLabel == label) {
                callback();
            }
        });
    }
    /**
     * @description 当用户点击托盘图标时触发该事件
     * @param callback 回调函数
     * @returns 用于取消监视
     */
    export function event_click_tray(callback: Function) {
        return listen('click_tray', () => {
            callback();
        })
    }
    /**
     * @description 当窗口移动时触发
     * @param callback 回调函数
     * @returns 用于取消监视
     */
    export function event_move(callback: Function) {
        return listen('tauri://move', () => {
            callback();
        })
    }

    export function event_file_drag(callback: (files: Array<string>) => void) {
        return listen<Array<string>>('tauri://file-drop', (e) => {
            callback(e.payload);
        })
    }
}

export default Window;