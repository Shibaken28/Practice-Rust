use proconio::input;

fn divisor(n: i64) -> Vec<i64> {
    let mut res = Vec::new();
    let mut i : i64 = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
        i += 1;
    }
    res.sort();
    res
}


fn main() {
    input! {
        n: i64,
    }
    let res = divisor(n);
    for i in res {
        println!("{}", i);
    }
}

// verify https://atcoder.jp/contests/tessoku-book/submissions/44208043
