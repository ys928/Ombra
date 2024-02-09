use rgb::FromSlice;
use std::fs::File;
use std::io::BufWriter;

pub fn compress_jpg(img_path: &str, save_path: &str, quality: u32) -> bool {
    let ret = std::panic::catch_unwind(|| {
        let out_file = File::create(save_path).unwrap();
        let writer = BufWriter::new(&out_file);
        let mut cinfo = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
        let img = image::open(img_path).unwrap();
        cinfo.set_size(img.width() as usize, img.height() as usize);
        cinfo.set_quality(quality as f32);
        let mut comp = cinfo.start_compress(writer).unwrap();
        comp.write_scanlines(img.as_bytes()).unwrap();
        comp.finish().unwrap();
    });
    if ret.is_err() {
        return false;
    }
    return true;
}

pub fn compress_png(img_path: &str, save_path: &str, quality: u32) {
    let img = image::open(img_path).unwrap();
    let mut liq = imagequant::new();
    liq.set_quality(0, quality as u8).unwrap();
    let new_img = img.to_rgba8();
    let new_img = new_img.as_raw();
    let mut img = liq
        .new_image(
            new_img.as_rgba(),
            img.width() as usize,
            img.height() as usize,
            0.0,
        )
        .unwrap();
    let mut res = liq.quantize(&mut img).unwrap();
    res.set_dithering_level(1.0).unwrap();
    let (palette, pixels) = res.remapped(&mut img).unwrap();
    let mut enc = lodepng::Encoder::new();
    enc.set_palette(&palette).unwrap();
    enc.encode_file(
        save_path,
        &pixels,
        img.width() as usize,
        img.height() as usize,
    )
    .unwrap();
}
