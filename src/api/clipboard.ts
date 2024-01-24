import { clipboard } from "@tauri-apps/api";

export default class Clipboard {
    /**
     * 
     * @returns 返回剪切板中的文本
     */
    static get_text() {
        return clipboard.readText();
    }
    /**
     * 
     * @param text 设置剪切板的文本
     */
    static set_text(text: string) {
        clipboard.writeText(text);
    }
}