// 「ヒープデータを管理することが所有権の存在する理由」
// 所有権は値を破棄する権利と言い換えられる
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world");
    println!("Pointer: {:?}", s.as_ptr()); 
    println!("Length: {}", s.len());       
    println!("Capacity: {}", s.capacity());
    println!("{}",s);

    let x = String::from("hello");
    let y = x.clone();
    println!("{}",y);

    let hello = String::from("hello");
    takes_ownership(hello);

    let mut helloo = String::from("hello");

    //以下はエラーになる。引数で可変参照を定義してるから
    //change(hello);
    change(&mut helloo);
    println!("{}",helloo);
}
//s変数がスコープを抜けるとき(}に来たとき)Rustがdrop関数を呼び、メモリを返還する

fn takes_ownership (some_string:String) {
    println!("{}",some_string);
}

fn change (some_string: &mut String) {
    some_string.push_str(",world");
}