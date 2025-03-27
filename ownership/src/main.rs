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

}
//s変数がスコープを抜けるとき(}に来たとき)Rustがdrop関数を呼び、メモリを返還する

fn takes_ownership (some_string:String) {
    println!("{}",some_string);
}
