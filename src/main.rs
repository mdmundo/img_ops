use image::imageops::flip_horizontal;
use image::io::Reader;

fn main() {
    let img = Reader::open("test_image.png")
        .expect("Opening error")
        .decode()
        .expect("Decoding error");

    let edited_img = flip_horizontal(&img);

    edited_img
        .save("edited_test_image.png")
        .expect("Saving error");
}
