fn main() {
    let my_bio = ("Sameem", 7, "Rust", true);
    // one way to access the tuple
     println!("My Name is : {}", my_bio.0);
     println!("My Favorite number is : {}", my_bio.1);
     println!("My Favorite programming language : {}", my_bio.2);
     println!("Favorite programming language Rust ? : {}", my_bio.3);

     // other way to access tuple
     let (name, fav_number, fav_language, love_rust) = my_bio;
     println!("My Name is : {}", name);
     println!("My Favorite number is : {}", fav_number);
     println!("My Favorite programming language : {}", fav_language);
     println!("Favorite programming language Rust ? : {}", love_rust);
}
