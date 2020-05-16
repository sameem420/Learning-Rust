fn main() {
    let my_data = [1,2,3,4,5]; // array size and type is not defined
    for n in my_data.iter()
    {
        println!("{}",n);
    }

    let student_data:[u8;5] = [64,75,36,91,86]; // array size and type defined
    for n in 0..student_data.len()
    {
        println!("{}",student_data[n]);
    }
}
