import { clipboard } from "@tauri-apps/api";

namespace Clipboard {
    /**
     * 
     * @returns 返回剪切板中的文本
     */
    export function get_text() {
        return clipboard.readText();
    }
    /**
     * 
     * @param text 设置剪切板的文本
     */
    export function set_text(text: string) {
        clipboard.writeText(text);
    }
}

export default Clipboard;