import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { LogicalSize, WebviewWindow, appWindow } from "@tauri-apps/api/window";

export default class Window {
    /**
     * 当前窗口的label
     */
    static label = appWindow.label;
    /**
     * 关闭当前窗口
     */
    static close() {
        if (this.is_main()) {
            this.set_height(170);
            window.router.push('/');
        } else {
            appWindow.close();
        }
    }
    /**
     * @description 显示当前窗口
     */
    static show() {
        appWindow.show();
    }

    /**
     * 最小化当前窗口
     */
    static min() {
        appWindow.minimize();
    }

    /**
     * 切换到最大化或恢复正常
     */
    static toggle_max() {
        appWindow.toggleMaximize();
    }
    /**
     * @description 从主搜索面板窗口中新开一个app窗口（大部分情况下应仅限ombra内部使用）
     * @param appid app的id
     */
    static new_app(appid: string) {
        if (!this.is_main()) return;
        this.hide();
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
    static to_app(appid: string) {
        if (!this.is_main()) return;
        Window.set_height(600);
        Window.set_resizable(true);
        window.router.push(`/app?id=${appid}`);
    }
    /**
     * @description 将主窗口切换到主搜索界面
     */
    static to_main() {
        if (!this.is_main()) return;
        Window.set_height(170);
        Window.set_resizable(false);
        window.router.push(`/`);
    }
    /**
     * 
     * @returns 将主窗口切换到设置界面
     */
    static to_setting() {
        if (!this.is_main()) return;
        Window.set_height(600);
        Window.set_resizable(false);
        window.router.push(`/setting`);
    }

    /**
     * 窗口阴影、圆滑边框模式，仅限非主窗口使用
     */
    static shadow() {
        if (appWindow.label != 'MainWindow') {
            invoke('shadow_window');
        }
    }

    /**
     * @description 隐藏当前窗口
     */
    static hide() {
        appWindow.hide();
    }
    /**
     * @description 聚焦当前窗口
     */
    static focus() {
        appWindow.setFocus();
    }
    /**
     * 
     * @returns 当前窗口是否可见
     */
    static async is_visible() {
        return appWindow.isVisible();
    }

    /**
     * 
     * @returns 判断当前窗口是否是主搜索窗口
     */
    static is_main() {
        return appWindow.label == 'MainWindow';
    }

    /**
     * @description 设置当前窗口的大小
     * @param w 窗口的宽度，默认800
     * @param h 窗口的高度，默认主窗口高度170，app窗口推荐设置为600
     */
    static set_size(w = 800, h = 170) {
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
    static set_height(h = 170) {
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
    static set_resizable(resizable: boolean) {
        appWindow.setResizable(resizable);
    }

    /**
     * 
     * @param label 窗口label
     * @param callback 当该窗口失去聚焦时执行函数，
     */
    static event_blur(label: string, callback: Function) {
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
    static event_focus(label: string, callback: Function) {
        return listen('tauri://focus', (e) => {
            if (e.windowLabel == label) {
                callback();
            }
        });
    }
}