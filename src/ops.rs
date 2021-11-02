use image::imageops::{flip_horizontal, flip_vertical, rotate180, rotate270, rotate90};
use image::{io::Reader, DynamicImage, ImageBuffer, Rgba};
use std::vec::Vec;

pub fn open() -> DynamicImage {
    Reader::open("test_image.png")
        .expect("Opening error")
        .decode()
        .expect("Decoding error")
}

pub fn save(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    img.save("edited_test_image.png").expect("Saving error");
}

fn fh(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    flip_horizontal(img)
}

fn fv(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    flip_vertical(img)
}

fn r90(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    rotate90(img)
}

fn r180(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    rotate180(img)
}

fn r270(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    rotate270(img)
}

fn f(axis: &str, img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if axis.eq("h") {
        println!("Flip image horizontally");
        fh(img)
    } else {
        println!("Flip image vertically");
        fv(img)
    }
}

fn r(deg: &str, img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if deg.eq("90") {
        println!("Rotate image 90 degrees clockwise");
        r90(img)
    } else if deg.eq("180") {
        println!("Rotate image 180 degrees clockwise");
        r180(img)
    } else {
        println!("Rotate image 270 degrees clockwise");
        r270(img)
    }
}

pub fn op(op: &str, opt: &str, img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    if op.eq("f") {
        f(opt, img)
    } else {
        r(opt, img)
    }
}
