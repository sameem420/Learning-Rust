#[derive(Debug)]
struct Color {
    red:u8,
    green:u8,
    blue:u8
}
fn main() {
    let mut bg = Color {
        red:255,
        green:135,
        blue:45
    };

    println!("{:?}", bg);

    bg.blue = 63;
    bg.green = 127;

    println!("{}", bg.red);
    println!("{}", bg.green);
    println!("{}", bg.blue);
}
