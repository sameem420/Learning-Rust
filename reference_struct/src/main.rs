#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20
    };

    let result = print_area(&rect);
    println!("The Area of rectangle is : {}",result);
}

fn print_area(rect: &Rectangle) -> u32
{
    println!("{:#?}", rect);
    rect.height * rect.width
}
