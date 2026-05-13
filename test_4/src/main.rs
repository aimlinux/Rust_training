fn print_length(s: &String) {
    println!("長さ：{}文字", s.len());
}


fn main() {
    let s1 = String::from("おはようみあ");
    print_length(&s1);  // s1の参照を渡す
    println!("s1: {}", s1);
}
