import { invoke, os } from "@tauri-apps/api";

namespace CLI {
    /**
     * @description 启动可执行文件
     * @param path 可执行文件路径
     */
    export async function start_app(path: string) {
        if (await os.platform() == 'win32') {
            invoke('cmd_exec', { args: ['start', '', path] });
        }
    }
    /**
     * @description 用命令行执行一个命令，
     * @param command 要执行的命令，用数组的形式组织命令与各参数，比如 ls -l 的形式应该为：['ls','-l']
     */
    export function exec(command: Array<string>) {
        invoke('cmd_exec', { args: command });
    }
}

export default CLI;