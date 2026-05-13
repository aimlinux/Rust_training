fn main() {
    let s1 = String::from("わたしのみあ");
    let s2 = s1;

    // println!("{}", s1);  // エラー: s1はs2にムーブされているため、s1はもはや有効ではない。
    println!("{}", s2);  // 出力: わたしのみあ
}