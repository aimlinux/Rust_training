fn main() {

    let num = 7;
    let result = if num % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };

    println!("{}は{}です。", num, result);

    // loop(無限ループ)
    let mut count = 0;
    loop {
        count += 1;
        println!("ループの回数: {}", count);
        if count == 5 {
            break;
    }

    // for文
    for i in 0..5 {
        println!("{}", i)
    }

    // match文
    //（多言語のswith分に相当するが、はるかに強力）
    let value = 3;
    match value {
        1 => println!("one"),
        2 => println!("two"), 
        3 => println!("three"),
        _ => println!("other"),  // _はワイルドカード。どのパターンにもマッチしない場合に実行される。
    }
}