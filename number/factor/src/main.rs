use proconio::input;

fn factor(mut n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            res.push(i);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.push(n);
    }
    res
}

// p1^e1 * p2^e2 * ... * pk^ek -> [(p1, e1), (p2, e2), ..., (pk, ek)]
fn factor2(mut n: i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        let mut cnt = 0;
        while n % i == 0 {
            cnt += 1;
            n /= i;
        }
        if cnt != 0 {
            res.push((i, cnt));
        }
        i += 1;
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

fn main() {
    input! {
        n: i64,
    }
    let res = factor(n);
    for f in res {
        print!("{} ", f);
    }
    println!();
}

/*
fn main() {
    input! {
        n: i64,
    }
    let res = factor2(n);
    for (f, cnt) in res {
        for _ in 0..cnt {
            print!("{} ", f);
        }
    }
    println!();
}
*/


// verify https://atcoder.jp/contests/math-and-algorithm/submissions/44208636
// verify factor2 https://atcoder.jp/contests/math-and-algorithm/submissions/44208832
