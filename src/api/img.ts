import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

namespace Img {
    export async function compress(img_path: string, save_path: string, quality: Number) {
        let unlisten: UnlistenFn | undefined;
        const compress_result = new Promise<boolean>(async (resolve) => {
            //获取所有遍历到的程序
            unlisten = await listen<boolean>('img_compress_result', (e) => {
                resolve(e.payload)
            });
        });
        invoke('img_compress', { imgPath: img_path, savePath: save_path, quality: quality });
        let res = await compress_result;
        if (unlisten) {
            unlisten();
        }
        return res;
    }

    export async function convert(img_path: string, save_path: string, format: "bmp" | "jpg" | "png") {
        let unlisten: UnlistenFn | undefined;
        const convert_result = new Promise<boolean>(async (resolve) => {
            //获取所有遍历到的程序
            unlisten = await listen<boolean>('img_convert_result', (e) => {
                resolve(e.payload)
            });
        });
        invoke('img_convert', { imgPath: img_path, savePath: save_path, format: format })
        let res = await convert_result;
        if (unlisten) {
            unlisten();
        }
        return res;
    }
}

export default Img;