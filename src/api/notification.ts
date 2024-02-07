import { isPermissionGranted, requestPermission, sendNotification } from "@tauri-apps/api/notification";

namespace Notification {
    /**
     * 
     * @returns 当前是否允许发送提示消息
     */
    export async function is_grant() {
        return await isPermissionGranted();
    }
    /**
     * 
     * @returns 尝试请求获取发送系统提示的权限
     */
    export async function request() {
        const permission = await requestPermission();
        return permission === 'granted';
    }

    /**
     * @description 发送系统提示消息
     * @param title 标题
     * @param content 内容
     */
    export async function send(title: string, content: string) {
        sendNotification({ title: title, body: content });
    }

}

export default Notification;