import { invoke } from "@tauri-apps/api";

namespace Img {
    export async function compress(img_path: string, save_path: string, quality: Number) {
        let ret = await invoke<boolean>('img_compress', { imgPath: img_path, savePath: save_path, quality: quality });
        return ret;
    }

    export async function convert(img_path: string, save_path: string, format: "bmp" | "jpg" | "png") {
        let ret = await invoke<boolean>('img_convert', { imgPath: img_path, savePath: save_path, format: format })
        return ret;
    }
}

export default Img;