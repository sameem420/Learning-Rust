fn main() {
    let num:u32 = 5;
    let mut range:u32 = 1;
    let mut result:u32;
    loop {
        result = num * range;
        println!("{}{}{}{}{}", num , " x ", range, " = ", result);
        range = range + 1;
        if range > 10
        {
            break;
        }
    }
    
}
