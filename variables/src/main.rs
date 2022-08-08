fn main() {
    let s1 = String::from("zinzunghan");
    let (s2, len) = calculate_length(s1);
    println!("{},{}", s2, len);
}

fn calculate_length(s1:String) -> (String, usize) {
    let length = s1.len();
    (s1, length)
}
