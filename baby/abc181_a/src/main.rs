// if文の例
use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    /*
    if n % 2 == 0 {
        println!("White");
    } else {
        println!("Black");
    }
    */
    println!("{}", if n % 2 == 0 { "White" } else { "Black" });
}

// https://atcoder.jp/contests/abc181/submissions/44206228