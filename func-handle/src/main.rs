fn main() {
    another_function(5);
    print_labeled_measurement(5,'h');
    let  x = five();

    println!("x is {}",x);

    let y = plus_one(1);

    println!("y is {}",y)
}

fn another_function(x: i32) {
    println!("The value of x is {}",x)
}

fn print_labeled_measurement (x: i32,unit_label:char) {
    println!("The measurement is : {}{}",x,unit_label)
}

fn five() -> i32 {
    // セミコロンはつけない（つけると文扱いされ、値を返せなくなる。5は5に評価される式であることに注意）
    5
}

fn plus_one (x:i32) -> i32 {
    x + 1
}