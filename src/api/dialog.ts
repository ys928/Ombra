import { dialog } from "@tauri-apps/api";
import { OpenDialogOptions } from "@tauri-apps/api/dialog";

export default class Dialog {
    /**
     * @description 弹出一个确认对话框窗口
     * @param msg 窗口内容
     * @param title 窗口标题
     * @param type 类型
     * @param okLabel 确认按钮文本
     * @param cancelLabel 取消按钮文本
     * @returns true代表按下确认，否则代表按下取消
     */
    static confirm(msg: string, title: string, type: "warning" | "info" | "error", okLabel = '确定', cancelLabel = '取消') {
        return dialog.confirm(msg, { title: title, type: type, okLabel: okLabel, cancelLabel: cancelLabel })
    }
    /**
     * @description 弹出对话框选择文件打开
     * @param option 一些选项
     * @returns 返回选择结果路径，为null则说明没有选择
     */
    static open(option: OpenDialogOptions) {
        return dialog.open(option);
    }
}