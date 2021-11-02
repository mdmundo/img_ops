use image::io::Reader;

fn main() {
    let img = Reader::open("test_image.png")
        .expect("Opening error")
        .decode()
        .expect("Decoding error");

    // Operations

    img.save("edited_test_image.png").expect("Saving error");
}
