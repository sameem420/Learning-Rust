fn main() {
    check_even_odd(15);
}

fn check_even_odd(num:u32) {
    for i in 1..num{
        if i%2 == 0
        {
            println!("{} is even",i);
        }
        else
        {
            println!("{} is odd",i);
        }
    }
}
