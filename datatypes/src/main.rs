fn main() {
    // these are all scalar data types
    
    let fav_num:i16 = 7; // signed integer. for unsigned integer we use u8,u16 and so on.
    let any_num:f32 = 45.7; // floating point value
    let name = String::from("Sameem"); // string type value
    let fav_char:char = 'G'; // character type value
    let love_rust:bool = true; // boolean value

    println!("My name is : {}", name);
    println!("My favorite number is : {}", fav_num);
    println!("My favorite letter is : {}", fav_char);
    println!("Any Number : {}", any_num);
    println!("I love rust = {}", love_rust);
}
