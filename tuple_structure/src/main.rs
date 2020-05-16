#[derive(Debug)]
struct Color(u8,u8,u8);

fn main() {
    let red = Color(255,0,0);
    let green = Color(0,0,255);
    let blue = Color(0,255,0);

    println!("{:#?}",red);
    println!("{:#?}",green);
    println!("{:#?}",blue);
}
