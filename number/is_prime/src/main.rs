use proconio::input;


fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}


fn main() {
    input!{
        q: i32,
    }
    for _ in 0..q {
        input!{
            n: i32,
        }
        println!("{} ", if is_prime(n) { "Yes" } else { "No" });
    }
}

// verify https://atcoder.jp/contests/tessoku-book/submissions/44206828
