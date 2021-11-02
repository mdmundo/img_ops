mod ops;

fn main() {
    let img = ops::open();

    let mut args = std::env::args().skip(1);
    let op = args.next().expect("Operation required");
    let opt = args.next().expect("Option required");
    let edited_img = ops::op(&op, &opt, &img);

    ops::save(edited_img);
}
