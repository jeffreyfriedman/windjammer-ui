#![allow(dead_code)]
use std::path::PathBuf;
use wj_paint::Image;

pub fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("goldens").join(format!("{name}.png"))
}

pub fn write_png(path: &std::path::Path, img: &Image) {
    let file = std::fs::File::create(path).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), img.width, img.height);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&img.rgba).unwrap();
}

pub fn read_png(path: &std::path::Path) -> Option<Image> {
    let file = std::fs::File::open(path).ok()?;
    let dec = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = dec.read_info().ok()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).ok()?;
    buf.truncate(info.buffer_size());
    Some(Image { width: info.width, height: info.height, rgba: buf })
}

/// Pixels whose 3x3 neighbourhood is a single colour in `img` ("flat" interior pixels).
pub fn flat_mask(img: &Image) -> Vec<bool> {
    let (w, h) = (img.width as i32, img.height as i32);
    let mut m = vec![false; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            let c = img.pixel(x as u32, y as u32);
            let mut flat = true;
            'o: for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || ny < 0 || nx >= w || ny >= h {
                        continue;
                    }
                    if img.pixel(nx as u32, ny as u32) != c {
                        flat = false;
                        break 'o;
                    }
                }
            }
            m[(y * w + x) as usize] = flat;
        }
    }
    m
}
