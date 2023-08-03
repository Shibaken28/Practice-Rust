use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn gcd_n(a: &Vec<i64>) -> i64 {
    let mut res = a[0];
    for i in 1..a.len() {
        res = gcd(res, a[i]);
    }
    res
}

fn lcm_n(a: &Vec<i64>) -> i64 {
    let mut res = a[0];
    for i in 1..a.len() {
        res = lcm(res, a[i]);
    }
    res
}

fn main() {
    input!{
        n: usize,
        a: [i64; n],
    }
    println!("{}", lcm_n(&a));
}
