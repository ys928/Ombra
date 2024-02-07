import { path } from "@tauri-apps/api";

namespace Path {
    /**
     * 
     * @param paths 多个路径
     * @returns 返回拼接好的路径
     */
    export function join(...paths: string[]) {
        return path.join(...paths);
    }

    export function resolve(...paths: string[]) {
        return path.resolve(...paths);
    }
}

export default Path;