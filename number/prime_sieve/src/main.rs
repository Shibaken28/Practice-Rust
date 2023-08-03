use proconio::input;


fn prime_sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        if !is_prime[i] {
            continue;
        }
        is_prime[i] = true;
        let mut j = 2*i;
        while j <= n {
            is_prime[j] = false;
            j += i;
        }
    }
    is_prime
}


fn main() {
    input! {
        n: usize,
    }
    let is_prime = prime_sieve(n);
    for i in 0..=n {
        if is_prime[i] {
            println!("{}", i);
        }
    }
}


// verify https://atcoder.jp/contests/tessoku-book/submissions/44207557
